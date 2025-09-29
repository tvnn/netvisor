<script lang="ts">
	import type { Port } from '$lib/features/hosts/types/base';
	import { type FormApi } from '../../types';

	export let port: Port;
	export let onUpdate: (updates: Partial<Port>) => void = () => {};

	function handlePortNumberChange(event: Event) {
		const target = event.target as HTMLInputElement;
		const portNumber = parseInt(target.value) || 80;
		onUpdate({ number: portNumber });
	}

	function handleProtocolChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const protocol = target.value;
		onUpdate({ protocol });
	}
</script>

<div class="grid flex-1 grid-cols-2 gap-3">
	<div>
		<div class="mb-1 block text-xs font-medium text-gray-400">Port Number</div>
		<input
			type="number"
			min="1"
			max="65535"
			value={port.number}
			on:input={handlePortNumberChange}
			class="w-full rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
		/>
	</div>

	<div>
		<div class="mb-1 block text-xs font-medium text-gray-400">Protocol</div>
		<select
			value={port.protocol}
			on:change={handleProtocolChange}
			class="w-full rounded border border-gray-600 bg-gray-700 px-2 py-1 text-sm text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
		>
			<option value="Tcp">TCP</option>
			<option value="Udp">UDP</option>
		</select>
	</div>
</div>
