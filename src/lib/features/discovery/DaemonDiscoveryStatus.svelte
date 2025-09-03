<script lang="ts">
  import { X, Radar } from 'lucide-svelte';
	import { cancelDiscovery, initiateDiscovery } from '$lib/features/discovery/store';
	import type { DaemonDiscoveryUpdate } from '$lib/features/discovery/types/api';
	import type { Daemon } from '../daemons/types/base';

  export let daemon: Daemon;
  export let discoveryData: DaemonDiscoveryUpdate | null = null;

  $: isActive = discoveryData !== null;
  $: hasError = discoveryData?.error !== undefined && discoveryData?.error !== null;
  
  // Calculate progress across multiple subnets
  $: progressPercent = (() => {
    if (!isActive || !discoveryData || !discoveryData.total_subnets || discoveryData.total_subnets === 0) {
      return 0;
    }
    
    const currentSubnet = discoveryData.subnet || 0;
    const totalSubnets = discoveryData.total_subnets;
    const subnetProgress = (discoveryData.completed && discoveryData.total && discoveryData.total > 0) 
      ? (discoveryData.completed / discoveryData.total) 
      : 0;
    
    // Each subnet represents 1/total_subnets of the overall progress
    const subnetWeight = 1 / totalSubnets;
    const completedSubnetsProgress = currentSubnet * subnetWeight;
    const currentSubnetProgress = subnetProgress * subnetWeight;
    
    return Math.min(100, (completedSubnetsProgress + currentSubnetProgress) * 100);
  })();

  async function handleStartDiscovery() {
    if (!isActive) {
      await initiateDiscovery({daemon_id: daemon.id});
    }
  }

  async function handleCancelDiscovery() {
    if (isActive && discoveryData?.session_id) {
      await cancelDiscovery(discoveryData.session_id);
    }
  }
</script>

{#if isActive && discoveryData}
  <!-- Active Discovery Status -->
  <div class="flex items-center justify-between gap-3">
    <div class="flex-1 space-y-2">
      <div class="flex items-center gap-3">
        <span class="text-sm font-medium text-blue-400">{discoveryData.phase}</span>
        {#if discoveryData.discovered_count && discoveryData.discovered_count > 0}
          <span class="text-sm text-green-400 font-medium">{discoveryData.discovered_count} nodes found</span>
        {/if}
        {#if discoveryData.total_subnets > 1}
          <span class="text-sm text-gray-400">
            Subnet {(discoveryData.subnet || 0) + 1} of {discoveryData.total_subnets}
          </span>
        {/if}
      </div>
      
      {#if discoveryData.total_subnets && discoveryData.total_subnets > 0}
        <div class="flex items-center gap-2">
          <div class="flex-1 h-2 bg-gray-700 rounded-full overflow-hidden">
            <div 
              class="h-full bg-blue-500 transition-all duration-300 ease-out"
              style="width: {progressPercent}%"
            ></div>
          </div>
          <span class="text-xs text-gray-400">{Math.round(progressPercent)}%</span>
        </div>
      {/if}
    </div>

    {#if hasError}
      <div class="mt-1">
        <span class="text-red-600 text-sm">{discoveryData.error}</span>
      </div>
    {/if}
    
    <button 
      class="p-1 text-red-400 hover:text-red-300 hover:bg-gray-700 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      on:click={handleCancelDiscovery}
      title="Cancel Discovery"
    >
      <X class="w-4 h-4" />
    </button>
  </div>
{:else}
  <!-- Inactive - Start Discovery CTA -->
  <div class="flex justify-center">
    <button 
      class="flex items-center justify-center gap-2 px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed text-sm"
      on:click={handleStartDiscovery}
    >
      <Radar class="w-4 h-4" />
      <span>Discover Nodes</span>
    </button>
  </div>
{/if}