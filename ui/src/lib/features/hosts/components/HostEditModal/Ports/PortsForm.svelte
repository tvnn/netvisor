<script lang="ts">
	import { type Host } from '$lib/features/hosts/types/base';
	import { getServicesForPort } from '$lib/features/services/store';
	import { ports } from '$lib/shared/stores/metadata';
	import type { Port } from '$lib/features/hosts/types/base';
	import { PortTypeDisplay } from '$lib/shared/components/forms/selection/display/PortTypeDisplay.svelte';
	import { v4 as uuidv4 } from 'uuid';
	import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
	import { PortDisplayAlternate } from '$lib/shared/components/forms/selection/display/PortDisplayAlternate.svelte';

	export let formData: Host;

	// All ports are now on host.ports
	$: allPorts = (formData.ports || []).sort((a, b) => {
		// Sort by port number, then by protocol
		if (a.number !== b.number) {
			return a.number - b.number;
		}
		return a.protocol.localeCompare(b.protocol);
	});

	$: selectablePorts = ports
		.getItems()
		.filter(
			(p_type) =>
				p_type.metadata.can_be_added && !formData.ports.some((port) => port.type == p_type.id)
		);

	function handleAddPort(portId: string) {
		const formPorts = formData.ports;
		const portType = ports.getItem(portId);

		if (portType) {
			const newPort: Port = {
				number: portType.metadata.number,
				protocol: portType.metadata.protocol,
				type: portType.id,
				id: uuidv4()
			};
			formPorts.push(newPort);
		}

		formData.ports = [...formPorts];
	}

	function handleRemovePort(index: number) {
		const formPorts = formData.ports.filter((_, i) => i != index);
		formData.ports = [...formPorts];
	}
</script>

<div class="space-y-6 p-6">
	<ListManager
		label="Ports"
		helpText="Select a port to add to this host. To edit ports assigned to services, use the services tab."
		placeholder="Select an port to add..."
		emptyMessage="No ports on this host. Add one to get started."
		allowReorder={false}
		allowDuplicates={false}
		allowItemEdit={() => false}
		allowItemRemove={(port: Port) => getServicesForPort(port.id).length == 0}
		options={selectablePorts}
		items={allPorts}
		optionDisplayComponent={PortTypeDisplay}
		itemDisplayComponent={PortDisplayAlternate}
		onAdd={handleAddPort}
		onRemove={handleRemovePort}
		onEdit={() => {}}
	/>
</div>
