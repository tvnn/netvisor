<script lang="ts">
	import { optionsPanelExpanded, topologyOptions } from '../../store';
	import { networks } from '$lib/features/networks/store';
	import { ChevronDown, ChevronRight, Settings } from 'lucide-svelte';
	import OptionsCheckbox from './OptionsCheckbox.svelte';
	import ServiceCategoryMultiSelect from './ServiceCategoryMultiSelect.svelte';

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
<div class="topology-options absolute left-4 top-4 z-10 max-w-80 duration-300">
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
					<OptionsCheckbox
						bind:topologyOption={$topologyOptions.group_docker_bridges_by_host}
						title="Group Docker Bridges"
						description="Display Docker containers running on a single host in a single subnet grouping"
					/>
					<OptionsCheckbox
						bind:topologyOption={$topologyOptions.show_gateway_as_infra_service}
						title="Show Gateway as Infra"
						description="Display gateway services in the infra section of subnets they interface with"
					/>
					<OptionsCheckbox
						bind:topologyOption={$topologyOptions.show_interface_edges}
						title="Show Interface Edges"
						description="Show edges between interfaces that belong to the same host"
					/>
				</div>

				<!-- Infrastructure Service Categories -->
				<ServiceCategoryMultiSelect
					bind:topologyOption={$topologyOptions.infra_service_categories}
					onChange={handleInfraChange}
					title="Infrastructure Categories"
					description="Select service categories that should be displayed in the infra section of subnets they interface with"
				/>

				<!-- Hide Service Categories -->
				<ServiceCategoryMultiSelect
					bind:topologyOption={$topologyOptions.hide_service_categories}
					onChange={handleHideChange}
					title="Hide Categories"
					description="Select service categories that should be hidden"
				/>
			</div>
		{/if}
	</div>
</div>
