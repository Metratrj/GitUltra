<script lang="ts">
    import { cn } from "$lib/utils.js";


    import * as Tabs from "@/components/ui/tabs"
    export let value: string;
    export let onClick: (value: string) => void;

    let tabs: string[] = ["Home", "Dashboard"];

    export function registerTab(value: string) {
        tabs = [...tabs, value];
    }

    export function unregisterTab(value: string) {
        tabs = tabs.filter((tab) => tab !== value);
    }

    export function handleClick(value: string) {
        onClick(value);
    }
</script>

<Tabs.Root class="h-full space-y-6">
    <Tabs.List>
        {#each tabs as tab}
            <Tabs.Trigger value={tab} on:click={() => handleClick(tab)}>
                {tab}
            </Tabs.Trigger>
        {/each}
    </Tabs.List>
</Tabs.Root>