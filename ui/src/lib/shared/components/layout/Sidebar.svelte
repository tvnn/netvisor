<script lang="ts">
	import { entities } from '$lib/shared/stores/metadata';

	export let activeTab: string = 'hosts';
	export let onTabChange: (tab: string) => void;

	const navItems = [
		{ id: 'discovery', label: 'Discovery', icon: entities.getIconComponent('Discovery') },
		{ id: 'hosts', label: 'Hosts', icon: entities.getIconComponent('Host') },
		{ id: 'subnets', label: 'Subnets', icon: entities.getIconComponent('Subnet') },
		{ id: 'groups', label: 'Groups', icon: entities.getIconComponent('Group') },
		{ id: 'topology', label: 'Topology', icon: entities.getIconComponent('Topology') }
	];
</script>

<div class="sidebar">
	<!-- Logo/Brand -->
	<div class="sidebar-section">
		<h1 class="text-primary text-xl font-bold">NetVisor</h1>
	</div>

	<!-- Navigation -->
	<nav class="flex-1 p-4">
		<ul class="space-y-2">
			{#each navItems as item (item.id)}
				<li>
					<button
						on:click={() => onTabChange(item.id)}
						class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left font-medium transition-colors"
						class:bg-blue-700={activeTab === item.id}
						class:border={activeTab === item.id}
						class:border-blue-600={activeTab === item.id}
						class:text-white={activeTab === item.id}
						class:text-tertiary={activeTab !== item.id}
						class:hover:text-secondary={activeTab !== item.id}
						class:hover:bg-gray-800={activeTab !== item.id}
					>
						<svelte:component this={item.icon} class="h-5 w-5" />
						{item.label}
					</button>
				</li>
			{/each}
		</ul>
	</nav>
</div>
