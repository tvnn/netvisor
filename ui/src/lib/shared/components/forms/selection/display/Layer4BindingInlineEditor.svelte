<script lang="ts">
	import { formatInterface, getInterfaceFromId, getPortFromId } from '$lib/features/hosts/store';
	import { ALL_INTERFACES, type Host } from '$lib/features/hosts/types/base';
	import { getServicesForInterface, getServicesForPort } from '$lib/features/services/store';
	import type { Layer4Binding, Service } from '$lib/features/services/types/base';
	import { formatPort } from '$lib/shared/utils/formatting';

	export let binding: Layer4Binding;
	export let onUpdate: (updates: Partial<Layer4Binding>) => void = () => {};
	export let service: Service<Layer4Binding> | undefined = undefined;
	export let host: Host | undefined = undefined;

	// Sentinel value to represent "All Interfaces" (null interface_id)
	const ALL_INTERFACES_VALUE = '__ALL_INTERFACES__';

	$: port = getPortFromId(binding.port_id);
	$: iface = binding.interface_id ? getInterfaceFromId(binding.interface_id) : ALL_INTERFACES;

	// Type guard to check if a service has Layer4 bindings
	function isLayer4Service(svc: Service): svc is Service<Layer4Binding> {
		return svc.bindings.length === 0 || svc.bindings.every((b) => b.type === 'Layer4');
	}

	// Check which service (if any) has bound this port to a given interface
	// Returns the service that conflicts with binding this port to the given interface
	function getBindingService(portId: string, interfaceId: string | null): Service<Layer4Binding> | null {
		// Check OTHER services
		let allServices = getServicesForPort(portId).filter((s) => s.id !== service?.id);
		let otherServices = allServices.filter(isLayer4Service);
		
		for (let svc of otherServices) {
			// Check if this service has a binding that conflicts
			let hasConflict = svc.bindings.some((b) => {
				// If either binding is to ALL_INTERFACES (null), they conflict
				if (b.interface_id === null || interfaceId === null) {
					return true;
				}
				// Otherwise, they conflict only if they're the same specific interface
				return b.interface_id === interfaceId;
			});
			if (hasConflict) return svc;
		}

		// Check OTHER bindings in current service
		if (service) {
			let otherBindings = service.bindings.filter(
				(b) => b.id !== binding.id && b.port_id === portId
			);
			let hasConflict = otherBindings.some((b) => {
				// If either binding is to ALL_INTERFACES (null), they conflict
				if (b.interface_id === null || interfaceId === null) {
					return true;
				}
				// Otherwise, they conflict only if they're the same specific interface
				return b.interface_id === interfaceId;
			});
			if (hasConflict) return service;
		}

		return null;
	}

	// Create interface options with disabled state
	$: interfaceOptions = host?.interfaces.map((iface) => {
		const boundService = getBindingService(binding.port_id, iface.id);
		return {
			iface,
			disabled: boundService !== null && iface.id !== binding.interface_id,
			boundService
		};
	}) || [];

	// Check ALL_INTERFACES option
	$: allInterfacesOption = (() => {
		const boundService = getBindingService(binding.port_id, null);
		return {
			iface: ALL_INTERFACES,
			disabled: boundService !== null && binding.interface_id !== null,
			boundService
		};
	})();

	// Create port options with disabled state
	$: portOptions = host?.ports.map((p) => {
		const boundService = getBindingService(p.id, binding.interface_id);
		return {
			port: p,
			disabled: boundService !== null && p.id !== binding.port_id,
			boundService
		};
	}) || [];

	// Convert binding.interface_id to select value (null -> sentinel string)
	$: selectValue = binding.interface_id === null ? ALL_INTERFACES_VALUE : binding.interface_id;

	function handlePortChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const portId = target.value;
		onUpdate({ port_id: portId });
	}

	function handleInterfaceChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const value = target.value;
		// Convert sentinel string back to null
		const interfaceId: string | null = value === ALL_INTERFACES_VALUE ? null : value;
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
				{:else if host.interfaces.length > 0}
					<!-- Multiple interfaces - show as dropdown -->
					<select
						value={selectValue}
						on:change={handleInterfaceChange}
						class="w-full rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
					>
						<option value="" disabled>Select interface...</option>
						{#each interfaceOptions as { iface, disabled, boundService } (iface.id)}
							<option value={iface.id} {disabled}>
								{formatInterface(iface)}{disabled && boundService ? ` - bound by ${boundService.name}` : ''}
							</option>
						{/each}
						<option 
							value={ALL_INTERFACES_VALUE}
							disabled={allInterfacesOption.disabled}
						>
							{formatInterface(ALL_INTERFACES)}{allInterfacesOption.disabled && allInterfacesOption.boundService ? ` - bound by ${allInterfacesOption.boundService.name}` : ''}
						</option>
					</select>
				{/if}
			</div>

			<div class="flex-1">
				{#if host.ports.length === 0}
					<div
						class="rounded border border-yellow-600 bg-yellow-900/20 px-2 py-1 text-xs text-yellow-400"
					>
						No ports configured on host
					</div>
				{:else}
					<!-- Always show dropdown when there are ports, so users can see disabled options -->
					<select
						value={binding.port_id}
						on:change={handlePortChange}
						class="w-full rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
					>
						<option value="" disabled>Select port...</option>
						{#each portOptions as { port, disabled, boundService } (port.id)}
							<option value={port.id} {disabled}>
								{formatPort(port)}{disabled && boundService ? ` - bound by ${boundService.name}` : ''}
							</option>
						{/each}
					</select>
				{/if}
			</div>
		</div>
	{/if}
</div>