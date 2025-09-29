<script lang="ts">
	import type { ServiceBinding } from '$lib/features/hosts/types/base';
	import type { Interface } from '$lib/features/hosts/types/base';
	import { getServiceHost, services } from '$lib/features/services/store';
	import { formatId } from '$lib/shared/utils/formatting';

	export let serviceBinding: ServiceBinding;
	export let onUpdate: (updates: Partial<ServiceBinding>) => void = () => {};

	// Get the service for this binding
	$: service = $services.find((s) => s.id === serviceBinding.service_id);

	// Get interfaces that this service is bound to
	$: boundInterfaces = service
		? getServiceHost(service.id)?.interfaces.filter((iface) =>
				service?.interface_bindings.includes(iface.id)
			)
		: [];

	function handleInterfaceChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const interfaceId = target.value;
		onUpdate({ interface_id: interfaceId });
	}

	function getInterfaceDisplayName(iface: Interface): string {
		const parts = [];
		if (iface.name) parts.push(iface.name);
		if (iface.ip_address) parts.push(iface.ip_address);
		return parts.length > 0 ? parts.join(' - ') : formatId(iface.id);
	}
</script>

<div class="flex-1">
	<div class="mb-1 block text-xs font-medium text-gray-400">Interface Binding</div>

	{#if !service}
		<div class="rounded border border-red-600 bg-red-900/20 px-2 py-1 text-xs text-red-400">
			Service not found
		</div>
	{:else if boundInterfaces && boundInterfaces.length === 0}
		<div
			class="rounded border border-yellow-600 bg-yellow-900/20 px-2 py-1 text-xs text-yellow-400"
		>
			No interface bindings configured
		</div>
	{:else if boundInterfaces && boundInterfaces.length === 1}
		<!-- Single interface - show as read-only -->
		<div class="rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm text-gray-300">
			{getInterfaceDisplayName(boundInterfaces[0])}
		</div>
	{:else if boundInterfaces}
		<!-- Multiple interfaces - show as dropdown -->
		<select
			value={serviceBinding.interface_id}
			on:change={handleInterfaceChange}
			class="w-full rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
		>
			<option value="" disabled>Select interface...</option>
			{#each boundInterfaces as iface (iface.id)}
				<option value={iface.id}>
					{getInterfaceDisplayName(iface)}
				</option>
			{/each}
		</select>
	{/if}
</div>
