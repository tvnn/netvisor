<script lang="ts">
	import { getDaemons } from '$lib/features/daemons/store';
	import DiagnosticsTab from '$lib/features/diagnostics/components/DiagnosticsTab.svelte';
	import { getDiagnosticExecutions } from '$lib/features/diagnostics/store';
	import { getActiveDiscoverySessions, stopDiscoveryPolling } from '$lib/features/discovery/store';
	import NodeGroupTab from '$lib/features/node_groups/components/NodeGroupTab.svelte';
	import { getNodeGroups } from '$lib/features/node_groups/store';
	import NodeTab from '$lib/features/nodes/components/NodeTab.svelte';
	import { getNodes, startNodePolling, stopNodePolling } from '$lib/features/nodes/store';
	import Sidebar from '$lib/shared/components/layout/Sidebar.svelte';
	import { getRegistry } from '$lib/shared/stores/registry';
	import { onDestroy, onMount } from 'svelte';
  
  let activeTab = 'nodes';
  
  function handleTabChange(tab: string) {
    activeTab = tab;
  }

  onMount(async () => {
    // Load initial data
    await getRegistry();
    await getNodes();
    await getDaemons();
    await getNodeGroups();
    await getDiagnosticExecutions();
    
    // Start continuous node polling for real-time updates
    startNodePolling();
    
    // Check for any active discovery sessions and resume if found
    await getActiveDiscoverySessions();
  });

  onDestroy(() => {
    stopNodePolling();
    stopDiscoveryPolling();
  });
</script>

<div class="min-h-screen bg-gray-900 text-white flex">
  <!-- Sidebar -->
  <Sidebar {activeTab} onTabChange={handleTabChange} />
  
  <!-- Main Content -->
  <main class="flex-1 overflow-auto">
    <div class="p-8">
      {#if activeTab === 'nodes'}
        <NodeTab />
      {:else if activeTab === 'groups'}
        <NodeGroupTab />
      {:else if activeTab === 'diagnostics'}
        <DiagnosticsTab />
      {/if}
    </div>
  </main>
</div>

<style>
  :global(html) {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }
  
  :global(body) {
    margin: 0;
    padding: 0;
    background: #111827;
  }
</style>