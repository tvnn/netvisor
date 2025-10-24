<script lang="ts">
	import { formatInterface } from '$lib/features/hosts/store';
	import { ALL_INTERFACES, type Host } from '$lib/features/hosts/types/base';
	import { getServicesForPort } from '$lib/features/services/store';
	import type { PortBinding, Service } from '$lib/features/services/types/base';
	import { formatPort } from '$lib/shared/utils/formatting';

	export let binding: PortBinding;
	export let onUpdate: (updates: Partial<PortBinding>) => void = () => {};
	export let service: Service | undefined = undefined;
	export let host: Host | undefined = undefined;

	// Type guard for services with Port bindings
	function isServiceWithPortBindings(svc: Service): svc is Service {
		return svc.bindings.length === 0 || svc.bindings.every((b) => b.type === 'Port');
	}

	// Check if this port+interface combination conflicts with existing bindings
	function getConflictingService(portId: string, interfaceId: string | null): Service | null {
		// Check OTHER services
		const otherServices = getServicesForPort(portId)
			.filter((s) => s.id !== service?.id)
			.filter(isServiceWithPortBindings);

		for (const svc of otherServices) {
			const hasConflict = svc.bindings.some((b) => {
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
			const otherBindings = service.bindings.filter(
				(b) => b.type === 'Port' && b.id !== binding.id && b.port_id === portId
			);
			const hasConflict = otherBindings.some((b) => {
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
	$: interfaceOptions =
		host?.interfaces.map((iface) => {
			// Check for Interface binding conflict - can't add Port binding if THIS service has Interface binding here
			const thisServiceHasInterfaceBinding = service?.bindings.some(
				(b) => b.type === 'Interface' && b.interface_id === iface.id && b.id !== binding.id
			);
			if (thisServiceHasInterfaceBinding) {
				return {
					iface,
					disabled: true,
					reason: 'This service has an Interface binding here',
					boundService: service
				};
			}

			// Check for Port binding conflict
			const boundService = getConflictingService(binding.port_id, iface.id);
			return {
				iface,
				disabled: boundService !== null && iface.id !== binding.interface_id,
				reason: boundService ? `Port bound by ${boundService.name}` : null,
				boundService
			};
		}) || [];

	// Check ALL_INTERFACES option
	$: allInterfacesOption = (() => {
		const boundService = getConflictingService(binding.port_id, null);
		return {
			iface: ALL_INTERFACES,
			disabled: boundService !== null && binding.interface_id !== null,
			reason: boundService ? `Port bound by ${boundService.name}` : null,
			boundService
		};
	})();

	// Create port options with disabled state
	$: portOptions =
		host?.ports.map((p) => {
			const boundService = getConflictingService(p.id, binding.interface_id);
			return {
				port: p,
				disabled: boundService !== null && p.id !== binding.port_id,
				reason: boundService ? `Bound by ${boundService.name}` : null,
				boundService
			};
		}) || [];

	// Convert binding.interface_id to select value (null -> sentinel string)
	$: selectValue = binding.interface_id === null ? ALL_INTERFACES.name : binding.interface_id;

	function handlePortChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const portId = target.value;
		onUpdate({ port_id: portId });
	}

	function handleInterfaceChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const value = target.value;
		// Convert sentinel string back to null
		const interfaceId: string | null = value === ALL_INTERFACES.name ? null : value;
		onUpdate({ interface_id: interfaceId });
	}
</script>

<div class="flex-1">
	<div class="text-secondary mb-1 block text-xs font-medium">Port Binding</div>

	{#if !service}
		<div class="text-danger rounded border border-red-600 bg-red-900/20 px-2 py-1 text-xs">
			Service not found
		</div>
	{:else if !host}
		<div class="text-danger rounded border border-red-600 bg-red-900/20 px-2 py-1 text-xs">
			Host not found
		</div>
	{:else}
		<div class="flex gap-3">
			<div class="flex-1">
				{#if host.interfaces && host.interfaces.length === 0}
					<div
						class="rounded border border-yellow-600 bg-yellow-900/20 px-2 py-1 text-xs text-warning"
					>
						No interfaces configured on host
					</div>
				{:else if host.interfaces.length > 0}
					<!-- Multiple interfaces - show as dropdown -->
					<select
						value={selectValue}
						on:change={handleInterfaceChange}
						class="text-primary w-full rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
					>
						<option value="" disabled>Select interface...</option>
						{#each interfaceOptions as { iface, disabled, reason } (iface.id)}
							<option value={iface.id} {disabled}>
								{formatInterface(iface)}{disabled && reason ? ` - ${reason}` : ''}
							</option>
						{/each}
						<option value={ALL_INTERFACES.name} disabled={allInterfacesOption.disabled}>
							{formatInterface(ALL_INTERFACES)}{allInterfacesOption.disabled &&
							allInterfacesOption.reason
								? ` - ${allInterfacesOption.reason}`
								: ''}
						</option>
					</select>
				{/if}
			</div>

			<div class="flex-1">
				{#if host.ports.length === 0}
					<div
						class="rounded border border-yellow-600 bg-yellow-900/20 px-2 py-1 text-xs text-warning"
					>
						No ports configured on host
					</div>
				{:else}
					<!-- Always show dropdown when there are ports, so users can see disabled options -->
					<select
						value={binding.port_id}
						on:change={handlePortChange}
						class="text-primary w-full rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
					>
						<option value="" disabled>Select port...</option>
						{#each portOptions as { port, disabled, reason } (port.id)}
							<option value={port.id} {disabled}>
								{formatPort(port)}{disabled && reason ? ` - ${reason}` : ''}
							</option>
						{/each}
					</select>
				{/if}
			</div>
		</div>
	{/if}
</div>
