<script lang="ts">
	import { formatInterface, getInterfaceFromId, getPortFromId } from '$lib/features/hosts/store';
	import type { Host } from '$lib/features/hosts/types/base';
	import { getServicesForPort } from '$lib/features/services/store';
	import type { PortInterfaceBinding, Service } from '$lib/features/services/types/base';
	import { formatPort } from '$lib/shared/utils/formatting';

	export let binding: PortInterfaceBinding;
	export let onUpdate: (updates: Partial<PortInterfaceBinding>) => void = () => {};
	export let service: Service | undefined = undefined;
	export let host: Host | undefined = undefined;

	$: port = getPortFromId(binding.port_id);
	$: iface = getInterfaceFromId(binding.interface_id);

	// Filter out ports which are already bound to a service on selected interface
	// BUT include the current port (since we're editing this binding)
	$: selectablePorts =
		host?.ports.filter((p) => {
			// Always include the current port being edited
			if (p.id === binding.port_id) return true;

			// Check if this port is bound to the selected interface by OTHER services
			let otherServices = getServicesForPort(p.id).filter((s) => s.id !== service?.id);
			let otherServiceBoundIfaceIds = otherServices
				.flatMap((s) => s.bindings)
				.map((b) => b.interface_id);

			// Check if this port is bound to the selected interface by OTHER bindings in current service
			let currentServiceOtherBindings =
				service?.bindings.filter((b) => b.id !== binding.id && b.port_id === p.id) || [];
			let currentServiceBoundIfaceIds = currentServiceOtherBindings.map((b) => b.interface_id);

			// Combine both checks
			let allBoundIfaceIds = [...otherServiceBoundIfaceIds, ...currentServiceBoundIfaceIds];

			return !allBoundIfaceIds.includes(binding.interface_id);
		}) || [];

	function handlePortChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const portId = target.value;
		onUpdate({ port_id: portId });
	}

	function handleInterfaceChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const interfaceId = target.value;
		onUpdate({ interface_id: interfaceId });
	}
</script>

<div class="flex-1">
	<div class="mb-1 block text-xs font-medium text-gray-400">Interface Binding</div>

	{#if !service}
		<div class="rounded border border-red-600 bg-red-900/20 px-2 py-1 text-xs text-red-400">
			Service not found
		</div>
	{:else if !host}
		<div class="rounded border border-red-600 bg-red-900/20 px-2 py-1 text-xs text-red-400">
			Host not found
		</div>
	{:else}
		<div class="flex gap-3">
			<div class="flex-1">
				{#if host.interfaces && host.interfaces.length === 0}
					<div
						class="rounded border border-yellow-600 bg-yellow-900/20 px-2 py-1 text-xs text-yellow-400"
					>
						No interfaces configured on host
					</div>
				{:else if host.interfaces && host.interfaces.length === 1}
					<!-- Single interface - show as read-only -->
					<div class="rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm text-gray-300">
						{iface ? formatInterface(iface) : 'Unknown Interface'}
					</div>
				{:else if host.interfaces.length > 0}
					<!-- Multiple interfaces - show as dropdown -->
					<select
						value={binding.interface_id}
						on:change={handleInterfaceChange}
						class="w-full rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
					>
						<option value="" disabled>Select interface...</option>
						{#each host.interfaces as iface (iface.id)}
							<option value={iface.id}>
								{formatInterface(iface)}
							</option>
						{/each}
					</select>
				{/if}
			</div>

			<div class="flex-1">
				{#if selectablePorts.length === 0}
					{#if host.ports.length === 0}
						<div
							class="rounded border border-yellow-600 bg-yellow-900/20 px-2 py-1 text-xs text-yellow-400"
						>
							No ports configured on host
						</div>
					{:else}
						<div
							class="rounded border border-yellow-600 bg-yellow-900/20 px-2 py-1 text-xs text-yellow-400"
						>
							No unbound ports available on selected interface
						</div>
					{/if}
				{:else if selectablePorts.length === 1}
					<!-- Single port - show as read-only -->
					<div class="rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm text-gray-300">
						{port ? formatPort(port) : 'Unknown Port'}
					</div>
				{:else if selectablePorts.length > 0}
					<!-- Multiple ports - show as dropdown -->
					<select
						value={binding.port_id}
						on:change={handlePortChange}
						class="w-full rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
					>
						<option value="" disabled>Select port...</option>
						{#each selectablePorts as port (port.id)}
							<option value={port.id}>
								{formatPort(port)}
							</option>
						{/each}
					</select>
				{/if}
			</div>
		</div>
	{/if}
</div>
