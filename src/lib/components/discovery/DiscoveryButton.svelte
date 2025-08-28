<script lang="ts">
  import { ChevronDown, Search } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { daemons, daemonActions } from '$lib/components/daemons/store';
  import { session_id, discoveryActions } from './store';
  import { nodes } from '../nodes/store';
  import type { Daemon } from '../daemons/types';
  
  let isDropdownOpen: boolean = false;
  let selectedDaemon: Daemon | null = null;

  onMount(() => {
    daemonActions.loadDaemons();
  })
  
  $: isDiscovering = $session_id != null;

  $: hasAvailableDaemons = $daemons.length > 0

  $: daemonNode = selectedDaemon !== null ? $nodes.find(n => n.id === selectedDaemon?.node_id || null) : null;
  
  // Auto-select first daemon
  $: if (hasAvailableDaemons && !selectedDaemon) {
    selectedDaemon = $daemons[0];
  }
  
  function startDiscovery() {
    if (selectedDaemon && !isDiscovering) {
      discoveryActions.initiateDiscovery(selectedDaemon.id)
    }
  }
</script>

<div class="relative">
  <!-- Main discovery button -->
  <div class="flex">
    <button 
      on:click={startDiscovery}
      disabled={!hasAvailableDaemons || isDiscovering}
      class="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-l-lg hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex-1"
    >
      <Search class="w-4 h-4" />
      {isDiscovering ? 'Discovering...' : 'Discover Nodes'}
      {#if selectedDaemon}
        <span class="text-blue-200 text-sm ml-2">via {daemonNode?.name}</span>
      {/if}
    </button>
    
    <!-- Dropdown trigger -->
    <button
      on:click={() => isDropdownOpen = !isDropdownOpen}
      disabled={!hasAvailableDaemons || isDiscovering}
      class="px-2 py-2 bg-blue-600 border-l border-blue-500 rounded-r-lg hover:bg-blue-700 transition-colors disabled:opacity-50"
    >
      <ChevronDown class="w-4 h-4 transition-transform {isDropdownOpen ? 'rotate-180' : ''}" />
    </button>
  </div>
  
  <!-- Dropdown menu -->
  {#if isDropdownOpen && hasAvailableDaemons}
    <div class="absolute right-0 mt-1 w-64 bg-gray-700 border border-gray-600 rounded-md shadow-lg z-50">
      <div class="py-1">
        <div class="px-3 py-2 text-xs text-gray-400 font-medium">Select Daemon:</div>
        {#each $daemons as daemon}
          {@const node = $nodes.find(n => n.id == daemon.node_id)}
          <button
            on:click={() => {
              selectedDaemon = daemon;
              isDropdownOpen = false;
            }}
            class="w-full px-3 py-2 text-left hover:bg-gray-600 flex items-center gap-2
                   {selectedDaemon?.id === daemon.id ? 'bg-blue-600' : ''}"
          >
            <div class="w-2 h-2 bg-green-400 rounded-full"></div>
            <div class="flex-1">
              <div class="font-medium">{node?.name || 'Unknown'}</div>
              <div class="text-sm text-gray-400">{node?.target || 'Unknown'}</div>
            </div>
            {#if selectedDaemon?.id === daemon.id}
              <div class="w-2 h-2 bg-blue-400 rounded-full"></div>
            {/if}
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>