<script lang="ts">
	import type { Service } from '$lib/features/services/types/base';
	import { getServiceById, services } from '$lib/features/services/store';
	import { ServiceDisplay } from '$lib/shared/components/forms/selection/display/ServiceDisplay.svelte';
	import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
	import { serviceDefinitions } from '$lib/shared/stores/metadata';

	export let service: Service;
	export let onChange: (updatedService: Service) => void;

	$: serviceMetadata = serviceDefinitions.getItem(service.service_definition);
	$: containerIds = service.containers || [];
	$: managedContainers = containerIds
		.map((id) => getServiceById(id))
		.filter((s) => s !== undefined);

	// Filter out services on other hosts and already managed containers
	$: selectableContainers = $services.filter(
		(s) => s.host_id === service.host_id && s.id !== service.id && !containerIds.includes(s.id)
	);

	function handleAddContainer(containerId: string) {
		const updatedContainerIds = [...containerIds, containerId];
		const updatedService = {
			...service,
			containers: updatedContainerIds
		};
		onChange(updatedService);
	}

	function handleRemoveContainer(index: number) {
		const updatedContainerIds = containerIds.filter((_, i) => i !== index);
		const updatedService = {
			...service,
			containers: updatedContainerIds
		};
		onChange(updatedService);
	}
</script>

<div class="space-y-6">
	<ListManager
		label="Containers"
		helpText="Manage containers controlled by this {serviceMetadata?.name
			? serviceMetadata.name
			: ''} instance"
		placeholder="Add container service..."
		emptyMessage="No containers managed by this service yet. Add services that run in containers on this host."
		allowReorder={false}
		allowDuplicates={false}
		allowItemEdit={() => false}
		showSearch={true}
		options={selectableContainers}
		items={managedContainers}
		optionDisplayComponent={ServiceDisplay}
		itemDisplayComponent={ServiceDisplay}
		onAdd={handleAddContainer}
		onRemove={handleRemoveContainer}
	/>
</div>
