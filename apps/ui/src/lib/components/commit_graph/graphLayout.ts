import type { CommitNode } from '@gitultra/schemas/ts/gitultra/bindings';

export class CommitGraphLayout {
	private nodes: Map<string, { x: number; y: number }> = new Map();
	constructor(private commits: CommitNode[]) {
		this.initializePositions();
		this.simulate(100);
	}

	private initializePositions() {
		// Simple linear layout for now
		this.commits.forEach((commit, index) => {
			this.nodes.set(commit.oid, { x: 0, y: index * 100 });
		});
	}

	private 

	get positions() {
		return this.nodes;
	}
}
