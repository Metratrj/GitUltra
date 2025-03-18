<script lang="ts">
	import { cn } from '$lib/utils.js';
	import { loadRepo } from '@/git';

	import * as Tabs from '@/components/ui/tabs';

	export let value: string;

	import { Package2, Plus, type Icon as IconType } from 'lucide-svelte';

	type MenuTab = {
		name: string;
		href?: string;
		icon?: typeof IconType;
	};

	let tabs: MenuTab[] = [
		/* 		{
			name: '',
			href: '/',
			icon: Package2
		}, */
		{
			name: 'Home',
			href: '/home'
		},
		{
			name: 'Dashboard',
			href: '/dashboard'
		}
	];

	import { goto } from '$app/navigation';
	export function onClick(value: MenuTab) {
		console.log(value);
		if (value.href) goto(value.href);
		else goto('/repo/' + value.name);
	}

	export function registerTab(value: MenuTab) {
		tabs = [...tabs, value];
	}

	export function unregisterTab(value: MenuTab) {
		tabs = tabs.filter((tab) => tab.name !== value.name);
	}

	export function handleClick(value: MenuTab) {
		onClick(value);
	}

	async function reload_loaded_repos() {
		const store = await load('gitultra-tauri-store');
		const values = await store.get<[string]>('gitultra-loaded-repos');
		console.log(values);
		if (!values) return;
		values.forEach((path) => {
			const name = path.split('/').pop();
			if (!name) return;
			registerTab({ name });
		});
	}

	export async function handleAdd(event: Event) {
		event.preventDefault();
		console.info('Add Repo');

		let ret = await loadRepo();
		if (!ret) return;
		if (!Array.isArray(ret) || ret.length === 0) return;
		const path = ret[0];
		const name = path.split('/').pop();
		if (!name) return;
		registerTab({ name });
	}

	import { onMount } from 'svelte';
	import { load } from '@tauri-apps/plugin-store';

	onMount(async () => {
		reload_loaded_repos();
	});
</script>

<Tabs.Root class="h-full space-y-6">
	<Tabs.List>
		{#each tabs as tab}
			<Tabs.Trigger value={tab.name} on:click={() => handleClick(tab)}>
				{#if tab.icon}
					{@const Icon = tab.icon}
					<Icon class="h-6 w-6" />
				{/if}
				{tab.name}
			</Tabs.Trigger>
		{/each}
		<Tabs.Trigger value="add" on:click={handleAdd}>
			<Plus class="h-5 w-5" />
		</Tabs.Trigger>
	</Tabs.List>
</Tabs.Root>
