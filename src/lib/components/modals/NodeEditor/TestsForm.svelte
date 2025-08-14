<script lang="ts">
  import { Plus, Edit, Trash2, ArrowUp, ArrowDown } from 'lucide-svelte';
  import type { AssignedTest } from "$lib/types/nodes";
  import { getTestTypeDisplayName } from "$lib/types/tests";
  import { getCriticalityDisplayName, getCriticalityColor } from "$lib/types/nodes";
  
  export let tests: AssignedTest[];
  export let onEditTest: (test: AssignedTest, index: number) => void;
  export let onCreateTest: () => void;
  
  function removeTest(index: number) {
    if (confirm('Are you sure you want to remove this test?')) {
      tests = tests.filter((_, i) => i !== index);
    }
  }
  
  function moveTestUp(index: number) {
    if (index > 0) {
      const newTests = [...tests];
      [newTests[index - 1], newTests[index]] = [newTests[index], newTests[index - 1]];
      tests = newTests;
    }
  }
  
  function moveTestDown(index: number) {
    if (index < tests.length - 1) {
      const newTests = [...tests];
      [newTests[index], newTests[index + 1]] = [newTests[index + 1], newTests[index]];
      tests = newTests;
    }
  }
  
  function getTestDisplayInfo(test: AssignedTest) {
    const displayName = getTestTypeDisplayName(test.test_type);
    const criticalityName = getCriticalityDisplayName(test.criticality);
    const criticalityColor = getCriticalityColor(test.criticality);
    
    // Extract target/description from test config
    let target = '';
    const config = test.test_config;
    
    if ('Connectivity' in config) {
      target = config.Connectivity.target;
    } else if ('DirectIp' in config) {
      target = config.DirectIp.target;
    } else if ('Ping' in config) {
      target = config.Ping.target;
    } else if ('ServiceHealth' in config) {
      target = config.ServiceHealth.target;
    } else if ('DnsResolution' in config) {
      target = config.DnsResolution.domain;
    } else if ('VpnConnectivity' in config) {
      target = config.VpnConnectivity.target;
    } else if ('VpnTunnel' in config) {
      target = config.VpnTunnel.expected_subnet;
    }
    
    return {
      displayName,
      target,
      criticalityName,
      criticalityColor,
      interval: test.monitor_interval_minutes ? `${test.monitor_interval_minutes}m` : 'Manual',
      enabled: test.enabled
    };
  }
</script>

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <h3 class="text-lg font-medium text-white">Assigned Tests</h3>
    <button
      type="button"
      on:click={onCreateTest}
      class="flex items-center gap-2 px-3 py-1 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
    >
      <Plus size={16} />
      Add Test
    </button>
  </div>
  
  {#if tests.length > 0}
    <div class="space-y-2">
      {#each tests as test, index}
        {@const info = getTestDisplayInfo(test)}
        <div class="flex items-center gap-3 p-3 bg-gray-700/50 rounded-lg border border-gray-600">
          <!-- Order number -->
          <span class="text-gray-400 font-mono text-sm min-w-[2rem]">{index + 1}.</span>
          
          <!-- Test Info -->
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2 mb-1">
              <span class="font-medium text-white">
                {info.displayName}
              </span>
              <span class="text-xs px-2 py-1 rounded {info.criticalityColor} bg-gray-800">
                {info.criticalityName}
              </span>
              {#if !info.enabled}
                <span class="text-xs px-2 py-1 rounded text-gray-400 bg-gray-800">
                  Disabled
                </span>
              {/if}
            </div>
            <div class="flex items-center gap-4 text-sm text-gray-400">
              {#if info.target}
                <span>Target: {info.target}</span>
              {/if}
              <span>Interval: {info.interval}</span>
            </div>
          </div>
          
          <!-- Actions -->
          <div class="flex items-center gap-1">
            <button
              type="button"
              on:click={() => moveTestUp(index)}
              disabled={index === 0}
              class="p-1 text-gray-400 hover:text-white hover:bg-gray-600 rounded disabled:opacity-30 disabled:cursor-not-allowed"
              title="Move up"
            >
              <ArrowUp size={16} />
            </button>
            
            <button
              type="button"
              on:click={() => moveTestDown(index)}
              disabled={index === tests.length - 1}
              class="p-1 text-gray-400 hover:text-white hover:bg-gray-600 rounded disabled:opacity-30 disabled:cursor-not-allowed"
              title="Move down"
            >
              <ArrowDown size={16} />
            </button>
            
            <button
              type="button"
              on:click={() => onEditTest(test, index)}
              class="p-1 text-blue-400 hover:text-blue-300 hover:bg-blue-900/20 rounded"
              title="Edit test"
            >
              <Edit size={16} />
            </button>
            
            <button
              type="button"
              on:click={() => removeTest(index)}
              class="p-1 text-gray-400 hover:text-red-400 hover:bg-red-900/20 rounded"
              title="Remove test"
            >
              <Trash2 size={16} />
            </button>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="text-center py-8 bg-gray-700/20 rounded-lg border-2 border-dashed border-gray-600">
      <p class="text-gray-400">No tests assigned</p>
      <p class="text-gray-500 text-sm mt-1">Click "Add Test" to assign monitoring tests to this node</p>
    </div>
  {/if}
  
  {#if tests.length > 0}
    <p class="text-xs text-gray-400">
      Tests will be executed in the order shown. Use the arrow buttons to reorder.
    </p>
  {/if}
</div>