<script lang="ts">
	import ListConfigEditor from '$lib/shared/components/forms/selection/ListConfigEditor.svelte';
	import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
	import ServicesConfigPanel from './ServicesConfigPanel.svelte';
	import type { Service } from '$lib/features/services/types/base';
	import type { Host } from '$lib/features/hosts/types/base';
	import { serviceDefinitions } from '$lib/shared/stores/metadata';
	import { createDefaultService } from '$lib/features/services/store';
	import { ServiceDisplay } from '$lib/shared/components/forms/selection/display/ServiceDisplay.svelte';
	import { ServiceTypeDisplay } from '$lib/shared/components/forms/selection/display/ServiceTypeDisplay.svelte';
	import type { FormApi } from '$lib/shared/components/forms/types';
	import { pushError } from '$lib/shared/stores/feedback';

	export let formApi: FormApi;
	export let formData: Host;
	export let currentServices: Service[] = [];

	// Available service types for adding
	const availableServiceTypes =
		serviceDefinitions
			.getItems()
			?.filter((service) => service.metadata?.can_be_added !== false)
			.sort((a, b) => a.category.localeCompare(b.category, 'en')) || [];

	// Event handlers
	function handleAddService(serviceTypeId: string) {
		const serviceMetadata = serviceDefinitions.getItems()?.find((s) => s.id === serviceTypeId);
		if (!serviceMetadata) return;

		const defaultPorts = (serviceMetadata.metadata?.default_ports as string[]) || [];

		const newService: Service = createDefaultService(
			serviceTypeId,
			formData.id,
			serviceDefinitions.getName(serviceTypeId),
			defaultPorts
		);

		currentServices = [...currentServices, newService as Service];
	}

	function handleRemoveService(index: number) {
		currentServices = currentServices.filter((_, i) => i !== index);
	}

	function handleServiceChange(service: Service, index: number) {
		if (index >= 0 && index < currentServices.length) {
			const updatedServices = [...currentServices];

			updatedServices[index] = service;
			currentServices = updatedServices;
		} else {
			pushError('Invalid service index');
		}
	}

	function handleServiceReorder(fromIndex: number, toIndex: number) {
		if (fromIndex === toIndex) return;

		const updatedServices = [...currentServices];
		const [movedService] = updatedServices.splice(fromIndex, 1);
		updatedServices.splice(toIndex, 0, movedService);

		currentServices = updatedServices;
	}
</script>

<div class="space-y-6">
	<ListConfigEditor
		bind:items={currentServices}
		onChange={handleServiceChange}
		onReorder={handleServiceReorder}
	>
		<svelte:fragment
			slot="list"
			let:items
			let:onEdit
			let:highlightedIndex
			let:onMoveUp
			let:onMoveDown
		>
			<ListManager
				label="Services"
				helpText="Services define what this host provides to the network."
				placeholder="Select service type to add..."
				emptyMessage="No services configured yet. Add one to get started."
				allowReorder={true}
				options={availableServiceTypes}
				showSearch={true}
				{items}
				allowItemRemove={() => true}
				optionDisplayComponent={ServiceTypeDisplay}
				itemDisplayComponent={ServiceDisplay}
				onAdd={handleAddService}
				onRemove={handleRemoveService}
				{onMoveDown}
				{onMoveUp}
				{onEdit}
				{highlightedIndex}
			/>
		</svelte:fragment>

		<svelte:fragment slot="config" let:selectedItem let:onChange>
			{#if selectedItem}
				<ServicesConfigPanel
					{formApi}
					bind:formData
					service={selectedItem}
					onChange={(updatedService) => onChange(updatedService)}
					host_interfaces={formData.interfaces}
				/>
			{:else}
				<div class="flex min-h-0 flex-1 items-center justify-center text-gray-400">
					<div class="text-center">
						<div class="mb-2 text-lg">No service selected</div>
						<div class="text-sm">Select an service from the list to configure it</div>
					</div>
				</div>
			{/if}
		</svelte:fragment>
	</ListConfigEditor>
</div>
