<script lang="ts">
	import * as THREE from 'three';
	import { onMount } from 'svelte';
	import { commands, type CommitNode } from '@gitultra/schemas/ts/gitultra/bindings';
	import { CommitGraphLayout } from './graphLayout';
	import { OrbitControls } from 'three/examples/jsm/Addons.js';

	let layout: CommitGraphLayout;
	let canvas: HTMLCanvasElement;
	let commits: CommitNode[] = [];
	let renderer: THREE.WebGLRenderer;
	let scene: THREE.Scene;
	let camera: THREE.PerspectiveCamera;
	

	export let repo_name: string;

	// Fetch commit data
	onMount(async () => {
		console.info('Loading commit graph for: ', repo_name);
		let resp = await commands.getCommitGraph(repo_name);
		console.log(resp);
		if (resp.status != 'ok') return;

		commits = resp.data;
		layout = new CommitGraphLayout(commits);
		initScene();
	});

	function initScene() {
		// Set up the scene
		scene = new THREE.Scene();
		scene.background = new THREE.Color(0x000000);
		camera = new THREE.PerspectiveCamera(75, canvas.offsetWidth / canvas.offsetHeight, 0.1, 1000);
		renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true });

		// set camera position
		camera.position.z = 5;
		const controls = new OrbitControls(camera, renderer.domElement);
		controls.enableDamping = true;
		controls.dampingFactor = 0.05;
		
		const positions = layout.positions; // Get computed positions

		// Create positions array for Three.js
		const positionArray = new Float32Array(commits.length * 3);
		commits.forEach((commit, i) => {
			const pos = positions.get(commit.oid);
			if (!pos) return;
			positionArray[i * 3] = pos.x;
			positionArray[i * 3 + 1] = pos.y;
			positionArray[i * 3 + 2] = 0;
		});

		const geometry = new THREE.InstancedBufferGeometry();
		geometry.setAttribute('position', new THREE.BufferAttribute(positionArray, 3));

		const count = 50000;
		let material = new THREE.MeshBasicMaterial({ color: 0xffff00 });
		const mesh = new THREE.InstancedMesh(geometry, material, count);
		// Set positions using layout data
		const dummy = new THREE.Object3D();
		commits.forEach((commit, i) => {
			const pos = positions.get(commit.oid);
			if (!pos) return;
			dummy.position.set(pos.x, pos.y, 0);
			dummy.updateMatrix();
			mesh.setMatrixAt(i, dummy.matrix);
		});

		scene.add(mesh);
		// add commit nodes
		/* commits.forEach((commit, i) => {
            let geometry = new THREE.SphereGeometry(0.1, 32, 32);
            let sphere = new THREE.Mesh(geometry, material);
            sphere.position.set(i * 0.2, 0, 0);
            scene.add(sphere);
        }); */

		// animation loop
		function animate() {
			requestAnimationFrame(animate);
			renderer.render(scene, camera);
		}

		animate();
	}

	function drawEdges() {
		const material = new THREE.LineBasicMaterial({ color: 0xffffff });
		commits.forEach((commit) => {
			commit.parents.forEach((parentOid) => {
				const start = layout.positions.get(commit.oid);
				const end = layout.positions.get(parentOid);

				const geometry = new THREE.BufferGeometry().setFromPoints([
					new THREE.Vector3(start?.x, start?.y, 0),
					new THREE.Vector3(end?.x, end?.y, 0)
				]);

				scene.add(new THREE.Line(geometry, material));
			});
		});
	}
</script>

<canvas bind:this={canvas}></canvas>
