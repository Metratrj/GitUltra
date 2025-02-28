<script lang="ts">
	import "../app.css";
	import { Button } from "@/components/ui/button";
	import { ModeWatcher } from "mode-watcher";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";

	import Sun from "lucide-svelte/icons/sun";
	import Moon from "lucide-svelte/icons/moon";

	import { resetMode, setMode } from "mode-watcher";

	import Activity from "lucide-svelte/icons/activity";
	import ArrowUpRight from "lucide-svelte/icons/arrow-up-right";
	import CircleUser from "lucide-svelte/icons/circle-user";
	import CreditCard from "lucide-svelte/icons/credit-card";
	import DollarSign from "lucide-svelte/icons/dollar-sign";
	import Menu from "lucide-svelte/icons/menu";
	import Package2 from "lucide-svelte/icons/package-2";
	import Search from "lucide-svelte/icons/search";
	import Users from "lucide-svelte/icons/users";

	import * as Avatar from "@/components/ui/avatar";
	import { Badge } from "@/components/ui/badge";
	import * as Card from "@/components/ui/card";
	import { Input } from "@/components/ui/input";
	import * as Sheet from "@/components/ui/sheet";
	import * as Table from "@/components/ui/table";
	import * as Menubar from "@/components/ui/menubar";
	import * as Tabs from "@/components/ui/tabs";
	import type { PageData } from "./$types";
	let { children } = $props();
</script>

<ModeWatcher />
<div class="flex min-h-screen w-full flex-col">
	<header
		class="bg-background sticky top-0 flex h-16 items-center gap-4 border-b px-4 md:px-6"
	>
		<nav
			class="hidden flex-col gap-6 text-lg font-medium md:flex md:flex-row md:items-center md:gap-5 md:text-sm lg:gap-6"
		>
			<a
				href="##"
				class="flex items-center gap-2 text-lg font-semibold md:text-base"
			>
				<Package2 class="h-6 w-6" />
				<span class="sr-only">Acme Inc</span>
			</a>
			<a
				href="##"
				class="text-foreground hover:text-foreground transition-colors"
			>
				Dashboard
			</a>
			<a
				href="##"
				class="text-muted-foreground hover:text-foreground transition-colors"
			>
				Orders
			</a>
			<a
				href="##"
				class="text-muted-foreground hover:text-foreground transition-colors"
			>
				Products
			</a>
			<a
				href="##"
				class="text-muted-foreground hover:text-foreground transition-colors"
			>
				Customers
			</a>
			<a
				href="##"
				class="text-muted-foreground hover:text-foreground transition-colors"
			>
				Analytics
			</a>
		</nav>
		<Sheet.Root>
			<Sheet.Trigger asChild let:builder>
				<Button
					variant="outline"
					size="icon"
					class="shrink-0 md:hidden"
					builders={[builder]}
				>
					<Menu class="h-5 w-5" />
					<span class="sr-only">Toggle navigation menu</span>
				</Button>
			</Sheet.Trigger>
			<Sheet.Content side="left">
				<nav class="grid gap-6 text-lg font-medium">
					<a href="##" class="flex items-center gap-2 text-lg font-semibold">
						<Package2 class="h-6 w-6" />
						<span class="sr-only">Acme Inc</span>
					</a>
					<a href="##" class="hover:text-foreground"> Dashboard </a>
					<a href="##" class="text-muted-foreground hover:text-foreground">
						Orders
					</a>
					<a href="##" class="text-muted-foreground hover:text-foreground">
						Products
					</a>
					<a href="##" class="text-muted-foreground hover:text-foreground">
						Customers
					</a>
					<a href="##" class="text-muted-foreground hover:text-foreground">
						Analytics
					</a>
				</nav>
			</Sheet.Content>
		</Sheet.Root>
		<div class="flex w-full items-center gap-4 md:ml-auto md:gap-2 lg:gap-4">
			<form class="ml-auto flex-1 sm:flex-initial">
				<div class="relative">
					<Search
						class="text-muted-foreground absolute left-2.5 top-2.5 h-4 w-4"
					/>
					<Input
						type="search"
						placeholder="Search products..."
						class="pl-8 sm:w-[300px] md:w-[200px] lg:w-[300px]"
					/>
				</div>
			</form>
			<DropdownMenu.Root>
				<DropdownMenu.Trigger asChild let:builder>
					<Button builders={[builder]} variant="outline" size="icon">
						<Sun
							class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
						/>
						<Moon
							class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
						/>
						<span class="sr-only">Toggle theme</span>
					</Button>
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end">
					<DropdownMenu.Item on:click={() => setMode("light")}
						>Light</DropdownMenu.Item
					>
					<DropdownMenu.Item on:click={() => setMode("dark")}>Dark</DropdownMenu.Item>
					<DropdownMenu.Item on:click={() => resetMode()}>System</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
			<DropdownMenu.Root>
				<DropdownMenu.Trigger asChild let:builder>
					<Button
						builders={[builder]}
						variant="secondary"
						size="icon"
						class="rounded-full"
					>
						<CircleUser class="h-5 w-5" />
						<span class="sr-only">Toggle user menu</span>
					</Button>
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end">
					<DropdownMenu.Label>My Account</DropdownMenu.Label>
					<DropdownMenu.Separator />
					<DropdownMenu.Item>Settings</DropdownMenu.Item>
					<DropdownMenu.Item>Support</DropdownMenu.Item>
					<DropdownMenu.Separator />
					<DropdownMenu.Item>Logout</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		</div>
	</header>
</div>

{@render children()}
