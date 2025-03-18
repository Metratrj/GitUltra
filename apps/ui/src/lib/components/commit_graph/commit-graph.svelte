<script lang="ts">
	import * as THREE from 'three';
	import { onMount } from 'svelte';
	import { commands, type CommitNode } from '@gitultra/schemas/ts/gitultra/bindings';
	import { CommitGraphLayout } from './graphLayout';
	import { OrbitControls } from 'three/examples/jsm/Addons.js';
	import * as Table from '@/components/ui/table'

	let layout: CommitGraphLayout;
	let canvas: HTMLCanvasElement;
	let commits: CommitNode[] = [];
	let renderer: THREE.WebGLRenderer;
	let scene: THREE.Scene;
	let camera: THREE.OrthographicCamera;
	let mesh: THREE.InstancedMesh | null = null;
	let edges: THREE.LineSegments;

	let hoveredCommit: CommitNode | null = null;
	let mouseX = 0;
	let mouseY = 0;

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
		// Orthographic camera for 2D view
		const aspect = canvas.offsetWidth / canvas.offsetHeight;
		const frustumSize = 1000;
		camera = new THREE.OrthographicCamera(
			frustumSize * aspect / -2,
			frustumSize * aspect / 2,
			frustumSize / 2,
			frustumSize / -2,
			1,
			1000
		);
		renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true });

		// Camera setup
		camera.position.z = 5;
		const controls = new OrbitControls(camera, renderer.domElement);
		controls.enableDamping = true;
		controls.dampingFactor = 0.05;
		controls.enableRotate = false; // Disable 3D rotation
		controls.minDistance = 50; // Minimum zoom
		controls.maxDistance = 2000; // Maximum zoom

		// create nodes
		const positions = layout.positions; // Get computed positions
		const sphereGeometry = new THREE.SphereGeometry(0.5);
		// Create circle texture for sprites
		const createSpriteTexture = (color: number) => {
			const canvas = document.createElement('canvas');
			canvas.width = 64;
			canvas.height = 64;
			const ctx = canvas.getContext('2d')!;
			ctx.beginPath();
			ctx.arc(32, 32, 24, 0, 2 * Math.PI);
			ctx.fillStyle = `#${color.toString(16).padStart(6, '0')}`;
			ctx.fill();
			return new THREE.CanvasTexture(canvas);
		};

		const material = new THREE.SpriteMaterial({
			map: createSpriteTexture(0x4caf50),
			sizeAttenuation: false
		});

		// Create sprites instead of 3D meshes
		commits.forEach((commit, i) => {
			const sprite = new THREE.Sprite(material);
			const pos = layout.positions.get(commit.oid)!;
			sprite.position.set(pos.x, pos.y, 0);
			sprite.scale.set(40, 40, 1);
			scene.add(sprite);
		});
		const dummy = new THREE.Object3D();

		commits.forEach((commit, i) => {
			const pos = layout.positions.get(commit.oid)!;
			dummy.position.set(pos.x, pos.y, 0);
			dummy.updateMatrix();
			mesh.setMatrixAt(i, dummy.matrix);
		});

		scene.add(mesh);

		// Create edges
		const edgeMaterial = new THREE.LineBasicMaterial({ color: 0xffffff, opacity: 0.3 });
		const edgeGeometry = new THREE.BufferGeometry();
		const points: THREE.Vector3[] = [];

		commits.forEach((commit) => {
			commit.parents.forEach((parentOid) => {
				const start = layout.positions.get(commit.oid)!;
				const end = layout.positions.get(parentOid)!;
				points.push(new THREE.Vector3(start.x, start.y, 0));
				points.push(new THREE.Vector3(end.x, end.y, 0));
			});
		});

		edgeGeometry.setFromPoints(points);
		edges = new THREE.LineSegments(edgeGeometry, edgeMaterial);
		scene.add(edges);

		// Lighting
		const ambientLight = new THREE.AmbientLight(0x404040);
		const directionalLight = new THREE.DirectionalLight(0xffffff, 0.5);
		directionalLight.position.set(1, 1, 1);
		scene.add(ambientLight, directionalLight);

		/* // Create positions array for Three.js
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

		scene.add(mesh); */

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
			controls.update();
			renderer.render(scene, camera);
		}

		animate();
		window.addEventListener('resize', onResize);
	}

	function onResize() {
		const aspect = canvas.offsetWidth / canvas.offsetHeight;
		const frustumSize = 1000;
		camera.left = frustumSize * aspect / -2;
		camera.right = frustumSize * aspect / 2;
		camera.top = frustumSize / 2;
		camera.bottom = frustumSize / -2;
		camera.updateProjectionMatrix();
		renderer.setSize(canvas.offsetWidth, canvas.offsetHeight);
	}

	function onMouseMove(event: MouseEvent) {
		mouseX = event.clientX;
		mouseY = event.clientY;
		
		if (!mesh) return;

		const raycaster = new THREE.Raycaster();
		const mouse = new THREE.Vector2(
			(event.clientX / canvas.width) * 2 - 1,
			-(event.clientY / canvas.height) * 2 + 1
		);

		raycaster.setFromCamera(mouse, camera);
		const intersects = raycaster.intersectObject(mesh);

		hoveredCommit = intersects[0]?.instanceId !== undefined
			? commits[intersects[0].instanceId!]
			: null;
	}
</script>

<canvas bind:this={canvas} on:mousemove={onMouseMove}></canvas>
{#if hoveredCommit}
	<div class="tooltip">
		{hoveredCommit.author}: {hoveredCommit.message}
	</div>
{/if}

<Table.Root>
	<Table.Body>

		{#each commits as commit }
			<Table.Row>
				<Table.Cell>{commit.message}</Table.Cell>
			</Table.Row>
		{/each}
	</Table.Body>
</Table.Root>