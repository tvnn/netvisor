<script lang="ts">
	import type { Service } from '$lib/features/services/types/base';
	import { getHostFromId, hosts } from '$lib/features/hosts/store';
	import { HostDisplay } from '$lib/shared/components/forms/selection/display/HostDisplay.svelte';
	import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
	import ConfigHeader from '$lib/shared/components/forms/config/ConfigHeader.svelte';
	import { serviceDefinitions } from '$lib/shared/stores/metadata';

	export let service: Service;
	export let onChange: (updatedService: Service) => void;

	$: serviceMetadata = serviceDefinitions.getItem(service.service_definition);
	$: vmIds = service.vms || [];
	$: managedVms = vmIds.map((id) => getHostFromId(id)).filter((h) => h !== undefined);

	// Filter out the parent host and already managed VMs
	$: selectableVms = $hosts.filter(
		(host) => service.host_id !== host.id && !vmIds.includes(host.id)
	);

	function handleAddVm(vmId: string) {
		const updatedVmIds = [...vmIds, vmId];
		const updatedService = {
			...service,
			vms: updatedVmIds
		};
		onChange(updatedService);
	}

	function handleRemoveVm(index: number) {
		const updatedVmIds = vmIds.filter((_, i) => i !== index);
		const updatedService = {
			...service,
			vms: updatedVmIds
		};
		onChange(updatedService);
	}
</script>

<div class="space-y-6">
	{#if serviceMetadata}
		<ConfigHeader
			title="{serviceMetadata.name} - Virtual Machines"
			subtitle="Manage VMs controlled by this {serviceMetadata.name} instance"
		/>
	{/if}

	<ListManager
		label="Managed Virtual Machines"
		helpText="Select which hosts are VMs managed by this service"
		placeholder="Add VM host..."
		emptyMessage="No VMs managed by this service yet. Add hosts that are VMs running on this hypervisor."
		allowReorder={false}
		allowDuplicates={false}
		showSearch={true}
		options={selectableVms}
		items={managedVms}
		optionDisplayComponent={HostDisplay}
		itemDisplayComponent={HostDisplay}
		onAdd={handleAddVm}
		onRemove={handleRemoveVm}
	/>
</div>
