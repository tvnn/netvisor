<script lang="ts">
  import type { Port } from '$lib/features/services/types/base';
  
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

<div class="flex-1 grid grid-cols-2 gap-3">
  <div>
    <div class="block text-xs font-medium text-gray-400 mb-1">Port Number</div>
    <input
      type="number"
      min="1"
      max="65535"
      value={port.number}
      on:input={handlePortNumberChange}
      class="w-full px-2 py-1 text-sm bg-gray-700 border border-gray-600 rounded text-white focus:ring-2 focus:ring-blue-500 focus:outline-none"
    />
  </div>
  
  <div>
    <div class="block text-xs font-medium text-gray-400 mb-1">Protocol</div>
    <select
      value={port.protocol}
      on:change={handleProtocolChange}
      class="w-full px-2 py-1 text-sm bg-gray-700 border border-gray-600 rounded text-white focus:ring-2 focus:ring-blue-500 focus:outline-none"
    >
      <option value="Tcp">TCP</option>
      <option value="Udp">UDP</option>
    </select>
  </div>
</div>