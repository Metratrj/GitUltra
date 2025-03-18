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
		quadtree = d3
			.quadtree<CNode>()
			.x((d) => d.x)
			.y((d) => d.y);
		// Initialize simulation and zoom
		initSimulation(width, height);
		initZoom();

		// Hover detection
		canvas.addEventListener('mousemove', handleMouseMove);

		loading = false;
	});
	interface Branch {
		id: string;
		start: string;
		end: string;
		color: string;
	}
	let branches: Branch[] = [];
	const childMap = new Map<string, string[]>();
	// Helper function
	function isAncestor(commit: string, ancestor: string): boolean {
		// Implement actual ancestor check using your commit data
		return nodes.find((n) => n.id === commit)?.parents.includes(ancestor) ?? false;
	}
	function commitToColor(commitHash: string): string {
		const hash = Array.from(commitHash).reduce((acc, char) => acc + char.charCodeAt(0), 0);
		return `hsl(${hash % 360}, 70%, 50%)`;
	}

	function detectBranches() {
		// Build child map
		childMap.clear();
		nodes.forEach((node) => {
			node.parents.forEach((parent) => {
				if (!childMap.has(parent)) childMap.set(parent, []);
				childMap.get(parent)!.push(node.id);
			});
		});

		// Detect branch points and merge points
		const branchPoints = new Map<string, string>();
		const mergePoints = new Map<string, string[]>();

		nodes.forEach((node) => {
			// Detect merges (multiple parents)
			if (node.parents.length > 1) {
				mergePoints.set(node.id, node.parents);
			}

			// Detect branch points (multiple children)
			const children = childMap.get(node.id) || [];
			if (children.length > 1) {
				children.forEach((child) => {
					branchPoints.set(child, node.id);
				});
			}
		});

		// Build branch objects
		branches = [];
		const branchColors = new Map<string, string>();

		nodes.forEach((node) => {
			// Only process branch starts
			if (branchPoints.has(node.id)) {
				const start = branchPoints.get(node.id)!;
				let current = node.id;
				let end = current;

				// Follow the branch until merge or end
				while (true) {
					const parents = nodes.find((n) => n.id === current)?.parents || [];
					if (parents.length !== 1) break;

					const next = parents[0];
					if (mergePoints.has(next)) {
						end = next;
						break;
					}
					current = next;
				}

				// Generate unique color per branch
				if (!branchColors.has(start)) {
					branchColors.set(start, commitToColor(start));
				}

				branches.push({
					id: `${start}-${end}`,
					start,
					end,
					color: branchColors.get(start)!
				});
			}
		});
		const branchLifetimes = new Map<string, { start: string; end?: string }>();
		nodes.forEach((node) => {
			// When a new branch starts
			if (branchPoints.has(node.id)) {
				const start = branchPoints.get(node.id)!;
				if (!branchLifetimes.has(start)) {
					branchLifetimes.set(start, { start: node.id });
				}
			}

			// When a branch ends (merge commit)
			if (node.parents.length > 1) {
				node.parents.forEach((parent) => {
					branchLifetimes.forEach((lifetime, key) => {
						if (lifetime.end === undefined && isAncestor(parent, lifetime.start)) {
							lifetime.end = node.id;
						}
					});
				});
			}
		});

		// Generate final branches
		branches = Array.from(branchLifetimes.entries()).map(([id, lifetime]) => ({
			id: `${lifetime.start}-${lifetime.end || lifetime.start}`,
			start: lifetime.start,
			end: lifetime.end || lifetime.start,
			color: commitToColor(lifetime.start)
		}));
	}
	const branchMap = new Map<string, number>(); // commit ID -> column

	function initSimulation(width: number, height: number) {
		viewportHeight = height;
		const availableColumns: number[] = [];
		const activeBranches = new Map<number, string>(); // column -> head commit
		let maxColumn = 0;

		// Build child map for branch detection
		const childMap = new Map<string, string[]>();
		nodes.forEach((node) => {
			node.parents.forEach((parent) => {
				if (!childMap.has(parent)) childMap.set(parent, []);
				childMap.get(parent)!.push(node.id);
			});
		});

		// Process nodes in reverse topological order (oldest first)
		const processedNodes = [...nodes].reverse();

		processedNodes.forEach((node) => {
			const children = childMap.get(node.id) || [];

			const parentColumns = node.parents
				.filter((p) => branchMap.has(p))
				.map((p) => branchMap.get(p)!);

			let column: number;

			// Merge commit handling - prioritize column 0 for main branch
			if (parentColumns.length > 1) {
            // Prefer column 0 if present in parents
            column = parentColumns.includes(0) ? 0 : 
                     Math.min(...parentColumns);
            
            // Release merged columns
            parentColumns.filter(c => c !== column).forEach(c => {
                if (activeBranches.delete(c)) {
                    availableColumns.push(c);
                    availableColumns.sort((a, b) => a - b); // Keep sorted
                }
            });
        }
			// Branch split handling
			else if (children.length > 1) {
				// First child stays in parent column, others get new columns
				column = branchMap.get(node.id) ?? availableColumns.pop() ?? maxColumn++;

				// Assign new columns to additional children
				children.slice(1).forEach((child) => {
					const newCol = availableColumns.pop() ?? maxColumn++;
					branchMap.set(child, newCol);
					activeBranches.set(newCol, child);
				});
			}
			// New branch creation
			else if (parentColumns.length === 0) {
				column = availableColumns.length > 0 ? availableColumns.shift()! : maxColumn++;

				// Always keep column 0 available for main branch
				if (column === 0 && nodes.some((n) => branchMap.get(n.id) === 0)) {
					column = availableColumns.shift() ?? maxColumn++;
				}
			}
			// Linear commit
			else {
				column = parentColumns[0];
			}

			// Update data structures
			if (parentColumns.length === 0 && column !== 0) {
				availableColumns.push(column);
			}

			// Update data structures
			branchMap.set(node.id, column);
			activeBranches.set(column, node.id);
			node.x = column * COLUMN_WIDTH + COLUMN_WIDTH / 2;
			node.y = nodes.indexOf(node) * NODE_HEIGHT;

			console.log(`Commit ${node.id.slice(0, 7)}: Column ${column}`);
			console.log('Parent columns:', parentColumns);
			console.log('Available columns:', availableColumns);
		});

		detectBranches();
		updateVisibleNodes();
		updateSpatialIndex();
		console.log('Branches:', branches);
		console.log('Active branches:', Array.from(activeBranches.entries()));
	}

	function initZoom() {
		const zoom = d3
			.zoom<HTMLCanvasElement, unknown>()
			.scaleExtent([1, 1])
			.translateExtent([
				[0, 0],
				[canvas.width, nodes.length * NODE_HEIGHT]
			])
			.on('zoom', (event) => {
				// Constrain to verical movement only
				transform = d3.zoomIdentity.translate(0, event.transform.y).scale(1);
				detectBranches(); // Update branches on view change
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
		visibleNodes = nodes.filter((node) => node.y > minY && node.y < maxY).slice(0, 1000);
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

		// Draw branches with different colors
		branches.forEach((branch) => {
			const startNode = nodes.find((n) => n.id === branch.start)!;
			const endNode = nodes.find((n) => n.id === branch.end)!;

			ctx.strokeStyle = branch.color;
			ctx.lineWidth = 2;
			ctx.beginPath();
			ctx.moveTo(startNode.x, startNode.y);
			ctx.lineTo(endNode.x, endNode.y);
			ctx.stroke();
		});

		// Draw links
		ctx.strokeStyle = 'rgba(75, 155, 255, 0.8)';
		ctx.lineWidth = 2;
		links.forEach((link) => {
			const source = nodes.find((n) => n.id === link.source)!;
			const target = nodes.find((n) => n.id === link.target)!;
			ctx.beginPath();
			ctx.moveTo(source.x, source.y);
			ctx.lineTo(target.x, target.y);
			ctx.stroke();
		});

		// Draw nodes
		ctx.fillStyle = '#4CAF51';
		nodes.forEach((node) => {
			ctx.beginPath();
			ctx.arc(node.x, node.y, 5, 0, 2 * Math.PI);
			ctx.fill();
		});

		// Draw merge commits differently
		ctx.fillStyle = '#FF5722';
		nodes
			.filter((node) => node.parents.length > 1)
			.forEach((node) => {
				ctx.beginPath();
				ctx.arc(node.x, node.y, 8, 0, 2 * Math.PI);
				ctx.fill();
			});

		// Draw branches with different colors
		branches.forEach((branch) => {
			const startNode = nodes.find((n) => n.id === branch.start)!;
			const endNode = nodes.find((n) => n.id === branch.end)!;

			ctx.strokeStyle = branch.color;
			ctx.beginPath();
			ctx.moveTo(startNode.x, startNode.y);
			ctx.lineTo(endNode.x, endNode.y);
			ctx.stroke();
		});

		// Draw merge connections
		ctx.strokeStyle = '#FF5722';
		ctx.lineWidth = 2;
		nodes
			.filter((n) => n.parents.length > 1)
			.forEach((node) => {
				node.parents.forEach((parentId) => {
					const parent = nodes.find((n) => n.id === parentId)!;
					if (branchMap.get(parentId)! !== branchMap.get(node.id)!) {
						ctx.beginPath();
						ctx.moveTo(parent.x, parent.y);
						ctx.lineTo(node.x, node.y);
						ctx.stroke();
					}
				});
			});

		ctx.restore();
	}

	function updateSpatialIndex() {
		quadtree = quadtree.addAll(visibleNodes);
	}

	function handleMouseMove(event: MouseEvent) {
		const [x, y] = d3.pointer(event);
		const inverted = transform.invert([x, y]);
		hoveredCommit = quadtree.find(inverted[0], inverted[1], 15 * 3)!;
	}

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
