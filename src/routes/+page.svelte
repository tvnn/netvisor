<script lang="ts">
  import { onMount } from 'svelte';
  import NodesTab from '../lib/components/tabs/NodesTab.svelte';
  import DiagnosticsTab from '../lib/components/tabs/DiagnosticsTab.svelte';
  import Sidebar from '../lib/components/common/Sidebar.svelte';
  import { nodeActions } from '../lib/stores/nodes';
  
  let activeTab = 'nodes';
  
  onMount(() => {
    nodeActions.loadNodes();
  });
  
  function handleTabChange(tab: string) {
    activeTab = tab;
  }
</script>

<div class="min-h-screen bg-gray-900 text-white flex">
  <!-- Sidebar -->
  <Sidebar {activeTab} onTabChange={handleTabChange} />
  
  <!-- Main Content -->
  <main class="flex-1 overflow-auto">
    <div class="p-8">
      {#if activeTab === 'nodes'}
        <NodesTab />
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