<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {Button} from "@/components/ui/button";
    import {Input} from "@/components/ui/input";
    import {commands} from "@gitultra/schemas/ts/gitultra/bindings";


    let name = $state("");
    let greetMsg = $state("");
    let greet2Msg = $state("");

    async function greet(event: Event) {
        event.preventDefault();
        greetMsg = await invoke("greet", {name});
    }

    async function greet2(event: Event) {
        event.preventDefault();
        greet2Msg = await commands.greet(name);
    }
</script>
<main class="container">
    <h1>Welcome to Tauri + Svelte + Specta</h1>
    <form class="row" onsubmit={greet}>
        <input id="greet-input" placeholder="Enter a name..." bind:value={name}/>
        <button type="submit">Greet</button>
    </form>
    <p>{greetMsg}</p>
    <br>
    <form class="row" onsubmit={greet2}>
        <Input id="greet-input" placeholder="Enter a name..." bind:value={name}/>
        <Button type="submit">Greet</Button>
    </form>
    <p>{greet2Msg}</p>

</main>
