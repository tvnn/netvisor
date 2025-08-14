<!-- src/lib/components/modals/NodeEditor/BasicNodeForm.svelte -->
<script lang="ts">
  import type { NodeType } from "$lib/types/nodes";
  import { getNodeTypeDisplayName } from "$lib/types/nodes";
  
  export let formData: any;
  export let errors: Record<string, string>;
  
  const nodeTypes: NodeType[] = [
    'Router', 'Switch', 'AccessPoint', 'Firewall',
    'WebServer', 'DatabaseServer', 'MediaServer', 'DnsServer', 'VpnServer', 'NasDevice',
    'Workstation', 'IotDevice', 'Printer', 'Camera', 'UnknownDevice'
  ];
</script>

<div class="space-y-4">
  <h3 class="text-lg font-medium text-white">Basic Information</h3>
  
  <!-- Name and Node Type -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <div>
      <label for="name" class="block text-sm font-medium text-gray-300 mb-1">
        Name *
      </label>
      <input
        id="name"
        name="name"
        bind:value={formData.name}
        type="text"
        required
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        class:border-red-500={errors.name}
      />
      {#if errors.name}
        <p class="text-red-400 text-xs mt-1">{errors.name}</p>
      {/if}
    </div>
    
    <div>
      <label for="node_type" class="block text-sm font-medium text-gray-300 mb-1">
        Node Type
      </label>
      <select
        id="node_type"
        name="node_type"
        bind:value={formData.node_type}
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        <option value="">Select node type</option>
        {#each nodeTypes as type}
          <option value={type}>{getNodeTypeDisplayName(type)}</option>
        {/each}
      </select>
    </div>
  </div>
  
  <!-- Connection Information -->
  <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
    <div>
      <label for="ip" class="block text-sm font-medium text-gray-300 mb-1">
        IP Address
      </label>
      <input
        id="ip"
        name="ip"
        bind:value={formData.ip}
        type="text"
        placeholder="192.168.1.100"
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>
    
    <div>
      <label for="domain" class="block text-sm font-medium text-gray-300 mb-1">
        Domain/Hostname
      </label>
      <input
        id="domain"
        name="domain"
        bind:value={formData.domain}
        type="text"
        placeholder="server.local"
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>
    
    <div>
      <label for="port" class="block text-sm font-medium text-gray-300 mb-1">
        Port
      </label>
      <input
        id="port"
        name="port"
        bind:value={formData.port}
        type="number"
        min="1"
        max="65535"
        placeholder="80"
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        class:border-red-500={errors.port}
      />
      {#if errors.port}
        <p class="text-red-400 text-xs mt-1">{errors.port}</p>
      {/if}
    </div>
  </div>
  
  <!-- Path and Monitoring -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <div>
      <label for="path" class="block text-sm font-medium text-gray-300 mb-1">
        Path
      </label>
      <input
        id="path"
        name="path"
        bind:value={formData.path}
        type="text"
        placeholder="/api"
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>
    
    <div>
      <label class="flex items-center space-x-2 pt-7">
        <input
          type="checkbox"
          name="monitoring_enabled"
          bind:checked={formData.monitoring_enabled}
          class="rounded bg-gray-700 border-gray-600 text-blue-600 focus:ring-blue-500"
        />
        <span class="text-sm font-medium text-gray-300">Enable Monitoring</span>
      </label>
    </div>
  </div>
  
  <!-- Description -->
  <div>
    <label for="description" class="block text-sm font-medium text-gray-300 mb-1">
      Description
    </label>
    <textarea
      id="description"
      name="description"
      bind:value={formData.description}
      rows="3"
      placeholder="Optional description..."
      class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
    ></textarea>
  </div>
</div>