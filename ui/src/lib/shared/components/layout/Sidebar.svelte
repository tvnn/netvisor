<script lang="ts">
	import { entities } from '$lib/shared/stores/metadata';

	export let activeTab: string = 'hosts';
	export let onTabChange: (tab: string) => void;

	const navItems = [
		{ id: 'hosts', label: 'Hosts', icon: entities.getIconComponent('Host') },
		{ id: 'subnets', label: 'Subnets', icon: entities.getIconComponent('Subnet') },
		{ id: 'groups', label: 'Groups', icon: entities.getIconComponent('Group') },
		{ id: 'topology', label: 'Topology', icon: entities.getIconComponent('Topology') }
	];
</script>

<div class="flex min-h-screen w-64 flex-col border-r border-gray-700 bg-gray-800">
	<!-- Logo/Brand -->
	<div class="border-b border-gray-700 p-6">
		<h1 class="text-xl font-bold text-white">NetVisor</h1>
	</div>

	<!-- Navigation -->
	<nav class="flex-1 p-4">
		<ul class="space-y-2">
			{#each navItems as item (item.id)}
				<li>
					<button
						on:click={() => onTabChange(item.id)}
						class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left transition-colors"
						class:bg-blue-600={activeTab === item.id}
						class:text-white={activeTab === item.id}
						class:text-gray-300={activeTab !== item.id}
						class:hover:text-white={activeTab !== item.id}
						class:hover:bg-gray-700={activeTab !== item.id}
					>
						<svelte:component this={item.icon} class="h-5 w-5" />
						{item.label}
					</button>
				</li>
			{/each}
		</ul>
	</nav>

	<!-- Footer -->
	<div class="border-t border-gray-700 p-4">
		<div class="flex items-center gap-2 text-xs text-gray-400">
			<div class="h-2 w-2 rounded-full bg-green-500"></div>
			<span>Server Running</span>
		</div>
	</div>
</div>
