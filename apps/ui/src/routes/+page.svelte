<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { Input } from "@/components/ui/input";
	import { commands } from "@gitultra/schemas/ts/gitultra/bindings";
	import { Button } from "@/components/ui/button";

	import { open } from "@tauri-apps/plugin-dialog";

	let name = $state("");
	let greetMsg = $state("");
	let greet2Msg = $state("");

	async function greet(event: Event) {
		event.preventDefault();
		greetMsg = await invoke("greet", { name });
	}

	async function greet2(event: Event) {
		event.preventDefault();
		greet2Msg = await commands.greet(name);
	}

	async function loadRepo() {
		const dir = await open({
			multiple: false,
			directory: true,
		});

		console.log(dir);


		if (dir) {
			await commands.openRepoDirectory(dir);
		}
	}
</script>


<main class="container">
	<h1>Welcome to Tauri + Svelte + Specta</h1>
	<form class="row" onsubmit={greet}>
		<input id="greet-input" placeholder="Enter a name..." bind:value={name} />
		<button type="submit">Greet</button>
	</form>
	<p>{greetMsg}</p>
	<br />
	<form class="row" onsubmit={greet2}>
		<Input id="greet-input" placeholder="Enter a name..." bind:value={name} />
		<Button type="submit">Greet</Button>
	</form>
	<p>{greet2Msg}</p>

	<Button on:click={loadRepo}>Load Repo</Button>
</main>
