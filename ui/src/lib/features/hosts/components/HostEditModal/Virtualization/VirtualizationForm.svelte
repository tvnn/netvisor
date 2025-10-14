<script lang="ts">
	import ListConfigEditor from '$lib/shared/components/forms/selection/ListConfigEditor.svelte';
	import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
	import type { Service } from '$lib/features/services/types/base';
	import { serviceDefinitions } from '$lib/shared/stores/metadata';
	import VmManagerConfigPanel from './VmManagerConfigPanel.svelte';
	import ContainerManagerConfigPanel from './ContainerManagerConfigPanel.svelte';
	import EntityConfigEmpty from '$lib/shared/components/forms/EntityConfigEmpty.svelte';
	import { VirtualizationManagerServiceDisplay } from '$lib/shared/components/forms/selection/display/ServiceDisplay copy.svelte';

	export let virtualizationManagerServices: Service[];
	export let onServiceChange: (service: Service, index: number) => void;

	function handleServiceChange(service: Service, index: number) {
		onServiceChange(service, index);
	}
</script>

<div class="space-y-6">
	<ListConfigEditor bind:items={virtualizationManagerServices} onChange={handleServiceChange}>
		<svelte:fragment slot="list" let:items let:onEdit let:highlightedIndex>
			<ListManager
				label="Virtualization Services"
				helpText="Services that manage virtual machines or containers on this host"
				emptyMessage="No virtualization services on this host."
				{items}
				allowItemRemove={() => false}
				allowReorder={false}
				allowAddFromOptions={false}
				options={[] as Service[]}
				itemDisplayComponent={VirtualizationManagerServiceDisplay}
				optionDisplayComponent={VirtualizationManagerServiceDisplay}
				{onEdit}
				{highlightedIndex}
			/>
		</svelte:fragment>

		<svelte:fragment slot="config" let:selectedItem let:onChange>
			{#if selectedItem}
				{@const virtualizationType = serviceDefinitions.getMetadata(
					selectedItem.service_definition
				).manages_virtualization}
				{#if virtualizationType === 'vms'}
					<VmManagerConfigPanel
						service={selectedItem}
						onChange={(updatedService) => onChange(updatedService)}
					/>
				{:else if virtualizationType === 'containers'}
					<ContainerManagerConfigPanel
						service={selectedItem}
						onChange={(updatedService) => onChange(updatedService)}
					/>
				{:else}
					<EntityConfigEmpty
						title="Unknown virtualization type"
						subtitle="This service has an unrecognized virtualization management type"
					/>
				{/if}
			{:else}
				<EntityConfigEmpty
					title="No service selected"
					subtitle="Select a virtualization service from the list to manage its VMs or containers"
				/>
			{/if}
		</svelte:fragment>
	</ListConfigEditor>
</div>
