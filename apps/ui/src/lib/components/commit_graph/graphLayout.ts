import type { CommitNode } from '@gitultra/schemas/ts/gitultra/bindings';

export class CommitGraphLayout {
	private nodes: Map<string, { x: number; y: number }> = new Map();
	constructor(private commits: CommitNode[]) {
		this.initializePositions();
		this.simulate();
	}

	private initializePositions() {
		let column = 0;
		const columnMap = new Map<string, number>();
		
		this.commits.forEach((commit, index) => {
			// Detect merge commits
			if (commit.parents.length > 1) {
				column++;
				columnMap.set(commit.oid, column);
			}
			// Carry parent column if exists
			else if (commit.parents.length === 1) {
				const parentCol = columnMap.get(commit.parents[0]) || column;
				column = parentCol;
			}
			
			this.nodes.set(commit.oid, {
				x: column * 120, // 120px spacing between columns
				y: index * 60    // 60px vertical spacing
			});
		});
	}

	private simulate() {
		// TODO: Implement force simulation for layout optimization
	}

	get positions() {
		return this.nodes;
	}
}
