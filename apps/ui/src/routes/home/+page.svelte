<script lang="ts">
	import { accounts, mails } from "./data.js";

	import Mail from "./(components)/mail.svelte";
    import type { PageData } from './$types';
    let { data }: { data: PageData } = $props();
</script>

<Mail
	{accounts}
	{mails}
	defaultLayout={data.layout}
	defaultCollapsed={data.collapsed}
	navCollapsedSize={4}
/>

