<script lang="ts">
	import { type Host } from '$lib/features/hosts/types/base';
	import { getServicesForPort } from '$lib/features/services/store';
	import { ports } from '$lib/shared/stores/metadata';
	import type { Port } from '$lib/features/hosts/types/base';
	import { PortTypeDisplay } from '$lib/shared/components/forms/selection/display/PortTypeDisplay.svelte';
	import { v4 as uuidv4 } from 'uuid';
	import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
	import { PortDisplay } from '$lib/shared/components/forms/selection/display/PortDisplay.svelte';

	export let formData: Host;

	let allPorts: Port[] = [];
	let previousPortsLength = 0;

	// Only sort when ports are added or removed, not during editing
	$: {
		const currentPorts = formData.ports || [];

		// Re-sort only if the number of ports changed (add/remove)
		// or if this is the initial load
		if (currentPorts.length !== previousPortsLength) {
			allPorts = [...currentPorts].sort((a, b) => {
				// Sort by port number, then by protocol
				if (a.number !== b.number) {
					return a.number - b.number;
				}
				return a.protocol.localeCompare(b.protocol);
			});
			previousPortsLength = currentPorts.length;
		} else {
			// During editing, just update the reference without re-sorting
			allPorts = [...currentPorts];
		}
	}

	$: selectablePorts = ports
		.getItems()
		.filter(
			(p_type) =>
				p_type.metadata.can_be_added && !formData.ports.some((port) => port.type == p_type.id)
		);

	function handleCreateNewPort() {
		const newPort = {
			id: uuidv4(),
			protocol: 'Tcp',
			number: Math.floor(Math.random() * 65535) + 1,
			type: 'Custom'
		} as Port;

		let formPorts = formData.ports;
		formPorts.push(newPort);
		formData.ports = [...formPorts];
	}

	function handleEditPort(port: Port, index: number) {
		const formPorts = formData.ports;
		formPorts[index] = port;
		formData.ports = [...formPorts];
	}

	function handleAddPort(portId: string) {
		const formPorts = formData.ports;
		const portType = ports.getItem(portId);

		if (portType) {
			const newPort: Port = {
				number: portType.metadata.number as number,
				protocol: portType.metadata.protocol as string,
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
		helpText="Manage ports for this host"
		placeholder="Add standard port..."
		emptyMessage="No ports on this host. Add one to get started."
		allowReorder={false}
		allowCreateNew={true}
		createNewLabel="Custom Port"
		allowDuplicates={false}
		allowItemEdit={(port) => port.type == 'Custom'}
		allowItemRemove={(port: Port) => getServicesForPort(port.id).length == 0}
		options={selectablePorts}
		items={allPorts}
		optionDisplayComponent={PortTypeDisplay}
		itemDisplayComponent={PortDisplay}
		onCreateNew={handleCreateNewPort}
		onAdd={handleAddPort}
		onRemove={handleRemovePort}
		onEdit={handleEditPort}
	/>
</div>
