<!-- src/lib/components/nodes/NodeCard.svelte -->
<script lang="ts">
  import { Edit, Settings, Trash2, Server, SquareActivity } from 'lucide-svelte';
    import type { Node } from "$lib/types/nodes";
    import { getNodeStatusDisplayName, getNodeStatusColor } from "$lib/types/nodes";
  import { getNodeTypeDisplayName } from "$lib/types/nodes";
  
  export let node: Node;
  export let groupNames: string[] = [];
  export let onEdit: (node: Node) => void = () => {};
  export let onDelete: (node: Node) => void = () => {};
  export let onAssignTest: (node: Node) => void = () => {};
  
  function handleEdit() {
    onEdit(node);
  }
  
  function handleAssignTest() {
    onAssignTest(node);
  }
  
  function handleDelete() {
    onDelete(node);
  }
  
  // Get the display status - monitoring status takes precedence if disabled
  function getDisplayStatus() {
    if (!node.monitoring_enabled) {
      return 'Monitoring Disabled';
    }
    return getNodeStatusDisplayName(node.current_status);
  }
  
  // Get the status color - gray for monitoring disabled, otherwise node status color
  function getDisplayStatusColor() {
    if (!node.monitoring_enabled) {
      return 'text-gray-400';
    }
    return getNodeStatusColor(node.current_status);
  }

</script>

<div class="bg-gray-800 rounded-lg p-6 border border-gray-700 hover:border-gray-600 transition-colors flex flex-col h-full">
  <!-- Header -->
  <div class="flex justify-between items-start mb-4">
    <div class="flex items-center space-x-3">
      <Server size={24} class="text-blue-400" />
      <div>
        <h3 class="text-lg font-semibold text-white">{node.name}</h3>
        <p class="text-sm text-gray-400">
          {node.node_type ? getNodeTypeDisplayName(node.node_type) : 'Unknown Device'}
        </p>
      </div>
    </div>
    <span class="text-sm font-medium {getDisplayStatusColor()}">
      {getDisplayStatus()}
    </span>
  </div>
  
  <!-- Content - grows to fill available space -->
  <div class="flex-grow space-y-3">
    <!-- Connection Info -->
    {#if node.ip || node.domain}
      <div class="text-sm text-gray-300">
        {#if node.ip}
          <span>IP: {node.ip}</span>
          {#if node.port}:{node.port}{/if}
        {:else if node.domain}
          <span>Domain: {node.domain}</span>
          {#if node.port}:{node.port}{/if}
        {/if}
      </div>
    {/if}
    
    <!-- Capabilities -->
    
      <div class="text-sm">
        <span class="text-gray-400">Capabilities:</span>
        {#if node.capabilities && node.capabilities.length > 0}
          <span class="ml-2">
            {#each node.capabilities as capability, i}
              <span class="inline-block bg-blue-900/30 text-blue-300 px-2 py-1 rounded text-xs mr-1 mb-1">
                {capability}
              </span>
            {/each}
          </span>
        {:else}
          <span class="text-gray-500">No capabilities assigned</span>
        {/if}
      </div>
    
    <!-- Groups -->
    
      <div class="text-sm">
        <span class="text-gray-400">Groups:</span>
        {#if groupNames.length > 0}
          <span class="ml-2">
            {#each groupNames as groupName, i}
              <span class="inline-block bg-green-900/30 text-green-300 px-2 py-1 rounded text-xs mr-1 mb-1">
                {groupName}
              </span>
            {/each}
          </span>
        {:else}
          <span class="text-gray-500">No groups assigned</span>
        {/if}
      </div>
    
    <!-- Tests -->
    <div class="text-sm">
      <span class="text-gray-400">Tests:</span>
      <span class="ml-2">
        {#if node.assigned_tests && node.assigned_tests.length > 0}
          <div class="mt-1 space-y-1">
            {#each node.assigned_tests as test}
              <div class="flex items-center justify-between">
                <span class="text-gray-300">{test.test_type}</span>
                <div class="flex items-center space-x-2">
                  <span class="text-xs px-2 py-1 rounded {
                    test.criticality === 'Critical' ? 'bg-red-900/30 text-red-300' :
                    test.criticality === 'Important' ? 'bg-yellow-900/30 text-yellow-300' :
                    'bg-blue-900/30 text-blue-300'
                  }">
                    {test.criticality}
                  </span>
                  {#if test.monitor_interval_minutes}
                    <span class="text-xs text-gray-500">
                      {test.monitor_interval_minutes}m
                    </span>
                  {/if}
                  {#if !test.enabled}
                    <span class="text-xs text-gray-500">(disabled)</span>
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        {:else}
          <span class="text-gray-500">No tests assigned</span>
        {/if}
      </span>
    </div>
  </div>
  
  <!-- Action Buttons -->
  <div class="flex justify-between items-center pt-4 mt-4 border-t border-gray-700">
    <div class="flex space-x-2">
      <button
        on:click={handleEdit}
        class="p-2 text-gray-400 hover:text-white hover:bg-gray-700 rounded transition-colors"
        title="Edit Node"
      >
        <Edit size={16} />
      </button>
      
      <button
        on:click={handleAssignTest}
        class="p-2 text-gray-400 hover:text-white hover:bg-gray-700 rounded transition-colors"
        title="Assign Test"
      >
        <SquareActivity size={16} />
      </button>
    </div>
    
    <button
      on:click={handleDelete}
      class="p-2 text-gray-400 hover:text-red-400 hover:bg-red-900/20 rounded transition-colors"
      title="Delete Node"
    >
      <Trash2 size={16} />
    </button>
  </div>
</div>

<style>
  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  button:disabled:hover {
    background-color: transparent;
    color: inherit;
  }
</style>