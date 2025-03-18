<script lang="ts">
	import { commands, type CommitNode } from '@gitultra/schemas/ts/gitultra/bindings';
	import { onDestroy, onMount } from 'svelte';
	import * as d3 from 'd3';

	export let repo_name: string;

	type _CNode = CommitNode & {
		x?: number;
		y?: number;
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
		nodes = resp.data.map((commit) => ({
			...commit,
			id: commit.oid
		}));

		links = resp.data.flatMap((commit) =>
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
		const nodeSpacing = width / (nodes.length +1);

		// Position nodes in a straight horizontal line
		nodes.forEach((node, i) => {
			node.x = nodeSpacing * (i + 1); // Evenly spaced along the X-axis
			node.y = height / 2; // centered vertically
			node.fx = node.x; // fix x position
			node.fy = node.y; // fix y position
		});

		// no simulation needed
		draw();

		/* const resolvedLinks = links
			.map((link) => ({
				source: nodes.find((n) => n.id == link.source)!,
				target: nodes.find((n) => n.id == link.target)!
			}))
			.filter((link) => link.source && link.target);

		const CHARGE_STRENGTH = -60; // reduce repulsion
		const LINK_DISTANCE = 100; // shorter links
		const FORCE_X_STRENGTH = 0.2; // stronger horizontal alignment
		const FORCE_Y_STRENGTH = 0.2; // stronger vertical alignment (hierachy)
		const COLLISION_RADIUS = 12; // prevent node overlap

		timestamps = nodes.map((n) => n.timestamp);
		minTimestamp = Math.min(...timestamps);
		maxTimestamp = Math.max(...timestamps);
		const maxY = height * 0.8;

		simulation = d3
			.forceSimulation<CNode>(nodes)
			.force('charge', d3.forceManyBody<CNode>().strength(CHARGE_STRENGTH).distanceMax(300))
			.force('link', d3.forceLink(resolvedLinks).distance(LINK_DISTANCE).strength(0.8))
			.force('center', d3.forceCenter(width / 2, height / 2))
			.force('x', d3.forceX(width / 2).strength(FORCE_X_STRENGTH)) // Weaker horizontal alignment
			.force(
				'y',
				d3
					.forceY<CNode>((d) => {
						//height / 2
						return maxY - ((d.timestamp - minTimestamp) * maxY) / (maxTimestamp - minTimestamp);
					})
					.strength(FORCE_Y_STRENGTH)
			) // Weaker vertical alignment
			.force('collision', d3.forceCollide(COLLISION_RADIUS))
			.alphaDecay(0.1)
			.alphaTarget(0.01)
			.on('tick', draw);

		nodes.forEach((node, i) => {
			node.x = width / 2 + (Math.random() - 0.5) * 50;
			node.y = height * 0.1 + i * 2;
		});

		// Warmup simulation
		simulation.stop();
		for (let i = 0; i < 300; i++) simulation.tick();
		simulation.restart(); */
	}

	function initZoom() {
		const zoom = d3
			.zoom<HTMLCanvasElement, unknown>()
			.scaleExtent([0.1, 8])
			.on('zoom', (event) => {
				transform = event.transform;
				draw();
			});

		d3.select<HTMLCanvasElement, unknown>(canvas).call(zoom).call(zoom.transform, d3.zoomIdentity);
	}

	function draw() {
		ctx.clearRect(0, 0, canvas.width, canvas.height);
		ctx.save();
		ctx.translate(transform.x, transform.y);
		ctx.scale(transform.k, transform.k);

		// Draw links
		ctx.strokeStyle = 'rgba(75, 155, 255, 0.8)';
		ctx.lineWidth = 2;
		links.forEach((link) => {
			const source = nodes.find((n) => n.id === link.source);
			const target = nodes.find((n) => n.id === link.target);
			if (
				source &&
				target &&
				source.x !== undefined &&
				source.y !== undefined &&
				target.x !== undefined &&
				target.y !== undefined
			) {
				ctx.beginPath();
				ctx.moveTo(source.x, source.y);
				ctx.lineTo(target.x, target.y);
				ctx.stroke();
			}
		});

		// Draw nodes
		ctx.fillStyle = '#4CAF50';
		nodes.forEach((node) => {
			ctx.beginPath();
			ctx.arc(node.x!, node.y!, 3, 0, 2 * Math.PI);
			ctx.fill();
		});

		ctx.restore();
	}

	function handleMouseMove(event: MouseEvent) {
		const [x, y] = d3.pointer(event);
		const inverted = transform.invert([x, y]);

		hoveredCommit =
			nodes.find((node) => Math.hypot(node.x! - inverted[0], node.y! - inverted[1]) < 5) || null;
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
		<div class="tooltip" style={`left: ${hoveredCommit.x!}px; top: ${hoveredCommit.y!}px`}>
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
