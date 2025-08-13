<script lang="ts">
  import { Server, Settings, Monitor, MonitorOff, Trash2, Edit } from 'lucide-svelte';
  import type { Node } from '../../types';
  import { getNodeTypeDisplayName, getStatusColor } from '../../types';

  export let node: Node;
  export let groupNames: string[] = [];
  export let onEdit: (node: Node) => void = () => {};
  export let onDelete: (node: Node) => void = () => {};
  export let onToggleMonitoring: (node: Node) => void = () => {};
  export let onAssignTest: (node: Node) => void = () => {};

  $: statusColor = getStatusColor(node.current_status);
  $: nodeTypeDisplay = node.node_type ? getNodeTypeDisplayName(node.node_type) : 'Unknown Device';
</script>

<div class="bg-gray-800 rounded-lg border border-gray-700 p-4 hover:border-gray-600 transition-colors">
  <!-- Header -->
  <div class="flex items-start justify-between mb-3">
    <div class="flex items-center gap-2">
      <Server class="w-5 h-5 text-blue-400" />
      <div>
        <h3 class="font-semibold text-white">{node.name}</h3>
        <p class="text-sm text-gray-400">{nodeTypeDisplay}</p>
      </div>
    </div>
    
    <!-- Status indicator -->
    <div class="flex items-center gap-1">
      <div class="w-2 h-2 rounded-full" class:bg-green-500={statusColor === 'green'} class:bg-yellow-500={statusColor === 'yellow'} class:bg-red-500={statusColor === 'red'} class:bg-gray-500={statusColor === 'gray'}></div>
      <span class="text-xs text-gray-400 capitalize">{node.current_status}</span>
    </div>
  </div>

  <!-- Connection details -->
  <div class="space-y-1 mb-3">
    {#if node.ip}
      <div class="text-sm text-gray-300">
        <span class="text-gray-400">IP:</span> {node.ip}
        {#if node.port}:{node.port}{/if}
      </div>
    {/if}
    
    {#if node.domain}
      <div class="text-sm text-gray-300">
        <span class="text-gray-400">Domain:</span> {node.domain}
      </div>
    {/if}
    
    {#if node.description}
      <div class="text-sm text-gray-300">
        <span class="text-gray-400">Description:</span> {node.description}
      </div>
    {/if}
  </div>

  <!-- Capabilities -->
  {#if node.capabilities.length > 0}
    <div class="mb-3">
      <div class="text-xs text-gray-400 mb-1">Capabilities:</div>
      <div class="flex flex-wrap gap-1">
        {#each node.capabilities.slice(0, 3) as capability}
          <span class="text-xs bg-blue-600/20 text-blue-300 px-2 py-1 rounded">
            {capability}
          </span>
        {/each}
        {#if node.capabilities.length > 3}
          <span class="text-xs text-gray-400">+{node.capabilities.length - 3} more</span>
        {/if}
      </div>
    </div>
  {/if}

  <!-- Assigned tests -->
  <div class="mb-3">
    <div class="text-xs text-gray-400 mb-1">Tests:</div>
    {#if node.assigned_tests.length > 0}
      <div class="flex flex-wrap gap-1">
        {#each node.assigned_tests.slice(0, 2) as test}
          <span class="text-xs bg-purple-600/20 text-purple-300 px-2 py-1 rounded">
            {test.test_type}
            {#if test.monitor_interval_minutes}
              <span class="text-purple-400">({test.monitor_interval_minutes}m)</span>
            {/if}
          </span>
        {/each}
        {#if node.assigned_tests.length > 2}
          <span class="text-xs text-gray-400">+{node.assigned_tests.length - 2} more</span>
        {/if}
      </div>
    {:else}
      <span class="text-xs text-gray-500">No tests assigned</span>
    {/if}
  </div>

  <!-- Monitoring status -->
  <div class="flex items-center gap-2 mb-3">
    {#if node.monitoring_enabled}
      <Monitor class="w-4 h-4 text-green-400" />
      <span class="text-xs text-green-400">Monitoring enabled</span>
    {:else}
      <MonitorOff class="w-4 h-4 text-gray-500" />
      <span class="text-xs text-gray-500">Monitoring disabled</span>
    {/if}
  </div>

  <!-- Groups -->
  {#if groupNames.length > 0}
    <div class="mb-3">
      <div class="text-xs text-gray-400 mb-1">Groups:</div>
      <div class="flex flex-wrap gap-1">
        {#each groupNames.slice(0, 2) as groupName}
          <span class="text-xs bg-green-600/20 text-green-300 px-2 py-1 rounded">
            {groupName}
          </span>
        {/each}
        {#if groupNames.length > 2}
          <span class="text-xs text-gray-400">+{groupNames.length - 2} more</span>
        {/if}
      </div>
    </div>
  {/if}

  <!-- Actions -->
  <div class="flex items-center justify-between pt-3 border-t border-gray-700">
    <div class="flex items-center gap-2">
      <button
        on:click={() => onEdit(node)}
        class="p-1 text-gray-400 hover:text-white transition-colors"
        title="Edit node"
      >
        <Edit class="w-4 h-4" />
      </button>
      
      <button
        on:click={() => onToggleMonitoring(node)}
        class="p-1 text-gray-400 hover:text-white transition-colors"
        title={node.monitoring_enabled ? 'Disable monitoring' : 'Enable monitoring'}
      >
        {#if node.monitoring_enabled}
          <MonitorOff class="w-4 h-4" />
        {:else}
          <Monitor class="w-4 h-4" />
        {/if}
      </button>
      
      <button
        on:click={() => onAssignTest(node)}
        class="p-1 text-gray-400 hover:text-white transition-colors"
        title="Assign test"
      >
        <Settings class="w-4 h-4" />
      </button>
    </div>
    
    <button
      on:click={() => onDelete(node)}
      class="p-1 text-red-400 hover:text-red-300 transition-colors"
      title="Delete node"
    >
      <Trash2 class="w-4 h-4" />
    </button>
  </div>
</div>

<style>
  .bg-green-500 { background-color: rgb(34 197 94); }
  .bg-yellow-500 { background-color: rgb(234 179 8); }
  .bg-red-500 { background-color: rgb(239 68 68); }
  .bg-gray-500 { background-color: rgb(107 114 128); }
</style>