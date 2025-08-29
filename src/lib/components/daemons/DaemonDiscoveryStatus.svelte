<script lang="ts">
  import { Search, X, Loader2, AlertTriangle } from 'lucide-svelte';
  import type { Daemon } from '../daemons/types';
  import type { Node } from '../nodes/types';
	import { discoveryActions } from '../discovery/store';


  export let daemon: Daemon;
  export let node: Node | null | undefined = undefined;
  export let sessionId: string | null = null;
  export let discoveryData: {
    phase: string;
    completed: number;
    total: number;
    discovered_count: number;
    error?: string | null;
  } | null = null;
  export let loading: boolean = false;
  export let showName: boolean = true;

  // Helper to get daemon display name (from node since daemon doesn't have name)
  $: daemonDisplayName = node ? node.name : `Daemon ${daemon.id.substring(0, 8)}`;
  $: isActive = sessionId !== null;
  $: hasError = discoveryData?.error !== undefined && discoveryData?.error !== null;
  $: progressPercent = discoveryData && discoveryData.total > 0 
    ? (discoveryData.completed / discoveryData.total) * 100 
    : 0;

  async function handleStartDiscovery() {
    if (!isActive && !loading) {
      await discoveryActions.initiateDiscovery(daemon.id);
    }
  }

  async function handleCancelDiscovery() {
    if (isActive && sessionId) {
      await discoveryActions.cancelDiscovery(sessionId);
    }
  }
</script>

<div class="flex gap-6 p-3">
{#if showName}
  <div class="flex flex-shrink">
    <div class="flex flex-col gap-1">
      <span class="font-medium text-white">{daemonDisplayName}</span>
      {#if node}
        <span class="text-sm text-gray-400">on {node.name}</span>
      {:else}
        <span class="text-sm text-gray-500 italic">Node not found</span>
      {/if}
    </div>
  </div>
{/if}

{#if isActive && discoveryData}
<div class="flex-grow">
    <!-- Active Discovery Status -->
    <div class="flex items-center justify-between gap-3">
    <div class="flex-1 space-y-2">
        <div class="flex items-center gap-3">
        <span class="text-sm font-medium text-blue-400">{discoveryData.phase}</span>
        {#if discoveryData.discovered_count > 0}
            <span class="text-sm text-green-400 font-medium">{discoveryData.discovered_count} nodes found</span>
        {/if}
        </div>
        
        {#if discoveryData.total > 0}
        <div class="flex items-center gap-2">
            <div class="flex-1 h-2 bg-gray-700 rounded-full overflow-hidden">
            <div 
                class="h-full bg-blue-500 transition-all duration-300 ease-out"
                style="width: {progressPercent}%"
            ></div>
            </div>
        </div>
        {/if}
    </div>

    <!-- {#if hasError}
        <div class="mt-1">
        <span class="text-red-600 text-sm">{discoveryData.error}</span>
        </div>
    {/if} -->

    <button 
        class="p-2 text-red-400 hover:text-red-300 hover:bg-gray-700 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        on:click={handleCancelDiscovery}
        disabled={loading}
        title="Cancel Discovery"
    >
        {#if loading}
        <Loader2 class="w-4 h-4 animate-spin" />
        {:else}
        <X class="w-4 h-4" />
        {/if}
    </button>
    </div>
</div>
{:else}
    <!-- Inactive - Start Discovery CTA -->
    <div class="flex-shrink">
        <button 
            class="flex items-center justify-center gap-2 w-full px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            on:click={handleStartDiscovery}
            disabled={loading}
        >
            {#if loading}
            <Loader2 class="w-4 h-4 animate-spin" />
            {:else}
            <Search class="w-4 h-4" />
            {/if}
            <span>Discover Nodes</span>
        </button>
    </div>
{/if}
</div>