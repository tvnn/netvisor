<script lang="ts">
	import type { Port } from '$lib/features/hosts/types/base';

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
		<div class="text-secondary mb-1 block text-xs font-medium">Port Number</div>
		<input
			type="number"
			min="1"
			max="65535"
			value={port.number}
			on:input={handlePortNumberChange}
			class="input-field"
		/>
	</div>

	<div>
		<div class="text-secondary mb-1 block text-xs font-medium">Protocol</div>
		<select value={port.protocol} on:change={handleProtocolChange} class="input-field">
			<option value="Tcp">TCP</option>
			<option value="Udp">UDP</option>
		</select>
	</div>
</div>
