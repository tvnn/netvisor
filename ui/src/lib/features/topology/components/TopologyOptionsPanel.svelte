<script lang="ts">
	import { optionsPanelExpanded, topologyOptions } from '../store';
	import { serviceDefinitions } from '$lib/shared/stores/metadata';
	import { networks } from '$lib/features/networks/store';
	import { ChevronDown, ChevronRight, Settings } from 'lucide-svelte';
	import { onMount } from 'svelte';

	// Get unique service categories
	let serviceCategories: string[] = [];

	onMount(() => {
		const items = serviceDefinitions.getItems() || [];
		const categoriesSet = new Set(items.map((i) => serviceDefinitions.getCategory(i.id)));
		serviceCategories = Array.from(categoriesSet)
			.filter((c) => c)
			.sort();
	});

	function handleNetworkChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const selectedOptions = Array.from(target.selectedOptions).map((opt) => opt.value);
		topologyOptions.update((opts) => {
			opts.network_ids = selectedOptions;
			return opts;
		});
	}

	function handleInfraChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const selectedOptions = Array.from(target.selectedOptions).map((opt) => opt.value);
		topologyOptions.update((opts) => {
			opts.infra_service_categories = selectedOptions;
			return opts;
		});
	}

	function handleHideChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const selectedOptions = Array.from(target.selectedOptions).map((opt) => opt.value);
		topologyOptions.update((opts) => {
			opts.hide_service_categories = selectedOptions;
			return opts;
		});
	}
</script>

<!-- Floating Panel -->
<div class="absolute left-4 top-4 z-10 max-w-80 duration-300">
	<div class="card p-0 shadow-lg">
		<!-- Toggle Button -->
		<button
			class="btn-icon w-full items-center rounded-2xl text-left"
			on:click={() => optionsPanelExpanded.set(!$optionsPanelExpanded)}
		>
			{#if $optionsPanelExpanded}
				<div class="flex w-full gap-2 p-3">
					<ChevronDown class="text-secondary h-5 w-5" />
					<Settings class="text-primary h-5 w-5" />
					<span class="text-primary text-sm font-medium">Options</span>
				</div>
			{:else}
				<div class="flex w-full gap-2 p-3">
					<ChevronRight class="text-secondary h-5 w-5" />
					<Settings class="text-primary h-5 w-5" />
				</div>
			{/if}
		</button>

		<!-- Content -->
		{#if $optionsPanelExpanded}
			<div class="space-y-4 border-t border-gray-700 p-3">
				<!-- Helper text -->
				<div class="rounded bg-gray-800/50 pt-2">
					<p class="text-tertiary text-[10px] leading-tight">
						Hold Ctrl (Windows/Linux) or Cmd (Mac) to select/deselect multiple options
					</p>
				</div>

				<!-- Network Selection -->
				<div class="space-y-1.5" style="display: none">
					<label for="network-select" class="text-primary text-s block font-medium">
						Networks
					</label>
					<select
						id="network-select"
						multiple
						on:change={handleNetworkChange}
						class="text-primary w-full rounded-md border border-gray-600 bg-gray-700 px-2 py-1.5 text-xs focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
					>
						{#each $networks as network (network.id)}
							<option
								value={network.id}
								selected={$topologyOptions.network_ids.includes(network.id)}
							>
								{network.name}{network.is_default ? ' (Default)' : ''}
							</option>
						{/each}
					</select>
				</div>

				<!-- Boolean Options -->
				<div class="space-y-3">
					<label class="flex flex-col gap-1">
						<div class="flex items-center gap-2">
							<input
								type="checkbox"
								bind:checked={$topologyOptions.group_docker_bridges_by_host}
								class="h-4 w-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-1 focus:ring-blue-500"
							/>
							<span class="text-primary text-sm">Group Docker Bridges</span>
						</div>
						<p class="text-tertiary text-xs leading-tight">
							Display Docker containers running on a single host in a single subnet grouping
						</p>
					</label>

					<label class="flex flex-col gap-1">
						<div class="flex items-center gap-2">
							<input
								type="checkbox"
								bind:checked={$topologyOptions.show_gateway_as_infra_service}
								class="h-4 w-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-1 focus:ring-blue-500"
							/>
							<span class="text-primary text-sm">Show Gateway as Infra</span>
						</div>
						<p class="text-tertiary text-xs leading-tight">
							Display gateway services in the infra section of subnets they interface with
						</p>
					</label>
				</div>

				<!-- Infrastructure Service Categories -->
				<div class="space-y-1.5">
					<label for="infra-categories" class="text-primary block text-sm font-medium">
						Infrastructure Categories
					</label>
					<p class="text-tertiary text-xs leading-tight">
						Select service categories that should be displayed in the infra section of subnets they
						interface with
					</p>
					<select
						id="infra-categories"
						multiple
						on:change={handleInfraChange}
						class="text-primary w-full rounded-md border border-gray-600 bg-gray-700 px-2 py-1.5 text-xs focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
					>
						{#each serviceCategories as category (category)}
							<option
								value={category}
								selected={$topologyOptions.infra_service_categories.includes(category)}
							>
								{category}
							</option>
						{/each}
					</select>
				</div>

				<!-- Hide Service Categories -->
				<div class="space-y-1.5">
					<label for="hide-categories" class="text-primary block text-sm font-medium">
						Hide Categories
					</label>
					<p class="text-tertiary text-xs leading-tight">
						Select service categories that should be hidden
					</p>
					<select
						id="hide-categories"
						multiple
						on:change={handleHideChange}
						class="text-primary w-full rounded-md border border-gray-600 bg-gray-700 px-2 py-1.5 text-xs focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
					>
						{#each serviceCategories as category (category)}
							<option
								value={category}
								selected={$topologyOptions.hide_service_categories.includes(category)}
							>
								{category}
							</option>
						{/each}
					</select>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	/* Ensure checkboxes are styled consistently */
	input[type='checkbox']:checked {
		background-color: rgb(37, 99, 235);
		border-color: rgb(37, 99, 235);
	}

	/* Style multi-select options */
	select[multiple] option {
		padding: 0.25rem 0.5rem;
		cursor: pointer;
	}

	select[multiple] option:checked {
		background-color: rgb(37, 99, 235);
		color: white;
	}

	/* Remove default select styling for multi-select */
	select[multiple] {
		scrollbar-width: thin;
		scrollbar-color: #4b5563 #1f2937;
	}

	select[multiple]::-webkit-scrollbar {
		width: 8px;
	}

	select[multiple]::-webkit-scrollbar-track {
		background: #1f2937;
	}

	select[multiple]::-webkit-scrollbar-thumb {
		background-color: #4b5563;
		border-radius: 4px;
	}
</style>
