<script lang="ts">
	import { commands, type CommitNode } from '@gitultra/schemas/ts/gitultra/bindings';
	import { onDestroy, onMount } from 'svelte';
	import * as d3 from 'd3';
	import { topologicalSort } from '@/utils';

	export let repo_name: string;

	type _CNode = CommitNode & {
		x: number;
		y: number;
		fx?: number | null;
		fy?: number | null;
		id: string;
	};

	type CNode = _CNode & d3.SimulationNodeDatum;

	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D;
	let transform = d3.zoomIdentity;
	let nodes: CNode[] = [];
	let links: Array<{ source: string; target: string }> = [];
	let hoveredCommit: CNode | null = null;
	let loading = true;

	let viewportHeight = 0;
	const NODE_HEIGHT = 50;
	const COLUMN_WIDTH = 100;
	let visibleNodes: CNode[] = [];

	let quadtree: d3.Quadtree<CNode>;

	onMount(async () => {
		if (!canvas) {
			console.error('Canvas element not found');
			return;
		}
		let tmp_ctx = canvas.getContext('2d');
		if (!tmp_ctx) {
			console.error('Failed to get canvas context');
			return;
		}

		ctx = tmp_ctx;

		console.info('Loading commit graph for: ', repo_name);
		let resp = await commands.getCommitGraph(repo_name);
		console.log(resp);
		if (resp.status != 'ok') return;
		nodes = topologicalSort(resp.data).map((commit) => ({
			...commit,
			id: commit.oid,
			x: 0,
			y: 0
		}));

		links = nodes.flatMap((commit) =>
			commit.parents.map((parent) => ({
				source: commit.oid,
				target: parent
			}))
		);
		console.log('Nodes:', nodes);
		console.log('Links:', links);

		// Check for invalid links
		const invalidLinks = links.filter(
			(link) => !nodes.some((n) => n.id === link.source) || !nodes.some((n) => n.id === link.target)
		);

		if (invalidLinks.length > 0) {
			console.warn('Invalid links:', invalidLinks);
		}

		loading = false;

		// Setup canvas
		const width = (canvas.width = canvas.offsetWidth);
		const height = (canvas.height = canvas.offsetHeight);

		// Initialize simulation and zoom
		initSimulation(width, height);
		initZoom();

		// Hover detection
		canvas.addEventListener('mousemove', handleMouseMove);

		loading = false;
	});

	function initSimulation(width: number, height: number) {
		viewportHeight = height;
		const sortedNodes = topologicalSort(nodes);
		const availableColumns: number[] = [];
		const branchMap = new Map<string, number>(); // commit id -> column

		// Grid layout algorith
		const columns = new Map<string, number>(); // branch name -> column index
		let currentColumn = 2;

		/* nodes.forEach((node, i) => {
			const isMerge = node.parents.length > 1;
			const isBranchStart = node.parents.length === 0;

			if (isBranchStart) {
				currentColumn++;
				columns.set(node.id, currentColumn);
			}

			// Assign grid position
			node.x = (columns.get(node.id) || 2) * COLUMN_WIDTH;
			node.y = i * NODE_HEIGHT;

			if (isMerge) {
				const parentColumns = node.parents.map((p) => columns.get(p)!);
				node.x = (parentColumns.reduce((a, b) => a + b, 0) / parentColumns.length) * COLUMN_WIDTH;
			}
		}); */

		nodes.forEach((node, i) => {
			const parentColumns = node.parents.map(p => branchMap.get(p)).filter(c => c !== undefined) as number[];

			// determine column
			let column: number;
			if (parentColumns.length == 0) {
				// root commit - new branch
				column = availableColumns.length > 0 ? availableColumns.shift()! : branchMap.size;
			}
			else if (parentColumns.length > 1){
				// merge commit -center between parents
				column = parentColumns.reduce((a,b) => a+b,0) / parentColumns.length;
				availableColumns.push(...parentColumns.slice(1));
				
			} else {
				// Linear commit - follow parent column
				column = parentColumns[0];
			}

			branchMap.set(node.oid, column);
			node.x = column * COLUMN_WIDTH + COLUMN_WIDTH / 2;
			node.y = i * NODE_HEIGHT;
		}); 

		// Set initial view position
		//transform = d3.zoomIdentity.translate(0, -nodes[0].y + height/2);
		quadtree = d3
			.quadtree<CNode>()
			.x((d) => d.x)
			.y((d) => d.y);
		updateVisibleNodes();
		updateSpatialIndex();
	}

	function initZoom() {
		const zoom = d3
			.zoom<HTMLCanvasElement, unknown>()
			.scaleExtent([1, 1])
			.on('zoom', (event) => {
				// Constrain to verical movement only
				transform = d3.zoomIdentity.translate(0, event.transform.y).scale(1);
				updateVisibleNodes();
				updateSpatialIndex();
				draw();
			});

		d3.select<HTMLCanvasElement, unknown>(canvas).call(zoom).call(zoom.transform, d3.zoomIdentity);
	}

	function updateVisibleNodes() {
		const buffer = NODE_HEIGHT * 5; // render 5 rows above/below
		// Calculate visible Y range
		const minY = -transform.y - buffer;
		const maxY = -transform.y + viewportHeight + buffer;

		// Get nodes in viewport with buffer
		visibleNodes = nodes.filter((node) => node.y > minY && node.y < maxY);
	}

	function draw() {
		ctx.clearRect(0, 0, canvas.width, canvas.height);
		ctx.save();
		ctx.translate(transform.x, transform.y);

		// Draw grid lines
		ctx.strokeStyle = 'rgba(255, 255, 255, 0.1)';
		ctx.lineWidth = 1;
		for (let x = COLUMN_WIDTH; x < canvas.width; x += COLUMN_WIDTH) {
			ctx.beginPath();
			ctx.moveTo(x, -transform.y);
			ctx.lineTo(x, -transform.y + viewportHeight);
			ctx.stroke();
		}

		// Draw links
		ctx.strokeStyle = 'rgba(75, 155, 255, 0.8)';
		ctx.lineWidth = 2;

		const visibleIds = new Set(visibleNodes.map((n) => n.oid));
		const visibleLinks = links.filter(
			(link) => visibleIds.has(link.source) || visibleIds.has(link.target)
		);
		visibleLinks.forEach((link) => {
			const source = nodes.find((n) => n.id === link.source)!;
			const target = nodes.find((n) => n.id === link.target)!;
			if (visibleNodes.includes(source) || visibleNodes.includes(target)) {
				ctx.beginPath();
				ctx.moveTo(source.x, source.y);
				ctx.lineTo(target.x, target.y);
				ctx.stroke();
			}
		});

		// Draw nodes
		ctx.fillStyle = '#4CAF51';
		visibleNodes.forEach((node) => {
			ctx.beginPath();
			ctx.arc(node.x!, node.y!, 15, 0, 2 * Math.PI);
			ctx.fill();
		});

		ctx.restore();
	}

	function updateSpatialIndex() {
		quadtree = quadtree.addAll(visibleNodes);
	}

	function handleMouseMove(event: MouseEvent) {
		const [x, y] = d3.pointer(event);
		//const transformed = d3.zoomIdentity.translate(transform.x, transform.y).invert([x, y]);
		const inverted = transform.invert([x, y]);

		/* console.log('Canvas Position:', { x, y });
		console.log('Graph Position:', inverted);
		console.log('Tooltip Position:', {
			x: inverted[0] + transform.x,
			y: inverted[1] + transform.y
		}); */

		hoveredCommit = quadtree.find(inverted[0], inverted[1], 15 * 2)!;
	}
	/* function getTooltipPosition(node: CNode) {
		const TOOLTIP_WIDTH = 300;
		const TOOLTIP_HEIGHT = 100;

		let left = node.x + transform.x;
		let top = node.y + transform.y;

		// Keep within viewport bounds
		if (left + TOOLTIP_WIDTH > window.innerWidth) {
			left = window.innerWidth - TOOLTIP_WIDTH;
		}
		if (top + TOOLTIP_HEIGHT > window.innerHeight) {
			top = window.innerHeight - TOOLTIP_HEIGHT;
		}

		return { left, top };
	} */

	function getTooltipPosition(node: CNode) {
    const TOOLTIP_OFFSET = 15;
    const canvasRect = canvas.getBoundingClientRect();
    
    return {
      left: node.x + transform.x + canvasRect.left + TOOLTIP_OFFSET,
      top: node.y + transform.y + canvasRect.top + TOOLTIP_OFFSET
    };
  }
	onDestroy(() => {
		canvas.removeEventListener('mousemove', handleMouseMove);
	});
</script>

<div class="graph-container">
	<canvas bind:this={canvas} class="graph-canvas"></canvas>
	{#if loading}
		<div class="loading">Loading commit graph...</div>
	{:else if hoveredCommit}
		<div
			class="tooltip"
			style={`left: ${getTooltipPosition(hoveredCommit).left}px; top: ${getTooltipPosition(hoveredCommit).top}px`}
		>
			<div class="tooltip-header">{hoveredCommit.id.slice(0, 7)}</div>
			<div class="tooltip-author">{hoveredCommit.author}</div>
			<div class="tooltip-message">{hoveredCommit.message}</div>
		</div>
	{/if}
</div>

<style>
	.graph-container {
		position: relative;
		width: 100%;
		height: 80vh;
		background: #1a1a1a;
		overflow: hidden;
	}

	.graph-canvas {
		width: 100%;
		height: 100%;
		cursor: move;
	}

	.loading {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		color: white;
	}

	.tooltip {
		position: absolute;
		background: rgba(0, 0, 0, 0.9);
		color: white;
		padding: 0.5rem 1rem;
		border-radius: 4px;
		pointer-events: none;
		transform: translate(-50%, -100%);
		max-width: 300px;
		font-size: 0.9rem;
	}

	.tooltip-header {
		color: #4caf50;
		font-weight: bold;
		margin-bottom: 0.3rem;
	}
</style>
