<script lang="ts">
	import { formatInterface, getInterfaceFromId } from '$lib/features/hosts/store';
	import type { Host } from '$lib/features/hosts/types/base';
	import type { Layer3Binding, Service } from '$lib/features/services/types/base';

	export let binding: Layer3Binding;
	export let onUpdate: (updates: Partial<Layer3Binding>) => void = () => {};
	export let service: Service | undefined = undefined;
	export let host: Host | undefined = undefined;

	$: iface = getInterfaceFromId(binding.interface_id);

	function handleInterfaceChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const interfaceId = target.value;
		onUpdate({ interface_id: interfaceId });
	}
</script>

<div class="flex-1">
	<div class="text-secondary mb-1 block text-xs font-medium">Interface Binding</div>

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
				{:else if host.interfaces && host.interfaces.length === 1}
					<!-- Single interface - show as read-only -->
					<div class="text-secondary rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm">
						{iface ? formatInterface(iface) : 'Unknown Interface'}
					</div>
				{:else if host.interfaces.length > 0}
					<!-- Multiple interfaces - show as dropdown -->
					<select
						value={binding.interface_id}
						on:change={handleInterfaceChange}
						class="text-primary w-full rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
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
		</div>
	{/if}
</div>
