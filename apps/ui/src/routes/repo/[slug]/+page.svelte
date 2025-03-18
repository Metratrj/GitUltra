<script lang="ts">
    import { CommitGraph } from '@/components/commit_graph';
import type { PageData } from './$types';

    let { data }: { data: PageData } = $props();
</script>

{#if data}
    <h1>{data.title}</h1>

    <CommitGraph repo_name={data.title} />
{/if}