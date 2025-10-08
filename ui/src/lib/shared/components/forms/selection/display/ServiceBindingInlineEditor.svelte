<script lang="ts">
	import type { ServiceBinding } from '$lib/features/hosts/types/base';
	import { getLayerBindingDisplayName, services } from '$lib/features/services/store';

	export let serviceBinding: ServiceBinding;
	export let onUpdate: (updates: Partial<ServiceBinding>) => void = () => {};

	// Get the service for this binding
	$: service = $services.find((s) => s.id === serviceBinding.service_id);

	function handleBindingChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const bindingId = target.value;
		onUpdate({ binding_id: bindingId });
	}
</script>

<div class="flex-1">
	<div class="mb-1 block text-xs font-medium text-gray-400">Interface Binding</div>

	{#if !service}
		<div class="rounded border border-red-600 bg-red-900/20 px-2 py-1 text-xs text-red-400">
			Service not found
		</div>
	{:else if service.bindings && service.bindings.length === 0}
		<div
			class="rounded border border-yellow-600 bg-yellow-900/20 px-2 py-1 text-xs text-yellow-400"
		>
			No interface bindings configured
		</div>
	{:else if service.bindings && service.bindings.length === 1}
		<!-- Single interface - show as read-only -->
		<div class="rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm text-gray-300">
			{getLayerBindingDisplayName(service.bindings[0])}
		</div>
	{:else if service.bindings}
		<!-- Multiple interfaces - show as dropdown -->
		<select
			value={serviceBinding.binding_id}
			on:change={handleBindingChange}
			class="w-full rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
		>
			<option value="" disabled>Select interface...</option>
			{#each service.bindings as binding (binding.id)}
				<option value={binding.id}>
					{getLayerBindingDisplayName(binding)}
				</option>
			{/each}
		</select>
	{/if}
</div>
