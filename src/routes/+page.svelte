<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api-client';
  
  // Components
  import Sidebar from '../lib/components/Sidebar.svelte';
  import DiagnosticsTab from '../lib/components/tabs/DiagnosticsTab.svelte';
  import NodesTab from '../lib/components/tabs/NodesTab.svelte';
  import TestsTab from '../lib/components/tabs/TestsTab.svelte';
  import Modal from '../lib/components/Modal.svelte';
  import Notifications from '../lib/components/Notifications.svelte';
  
  // Stores
  import { activeTab } from '../lib/stores/ui';
  import { nodes } from '../lib/stores/nodes';
  import { tests } from '../lib/stores/tests';
  
  // Initialize app
  onMount(async () => {
    try {
      // Load saved data from Tauri
      const [savedNodes, savedTests] = await Promise.all([
        api.getNodes(),
        api.getTests()
      ]);
      
      if (savedNodes) nodes.set(savedNodes);
      if (savedTests) tests.set(savedTests);
      
      console.log('App initialized successfully');
    } catch (error) {
      console.warn('Failed to load saved data:', error);
      // Continue with empty state - this is expected when running with stubs
    }
  });
</script>

<div class="min-h-screen bg-gray-900 text-white flex">
  <!-- Sidebar -->
  <Sidebar />
  
  <!-- Main Content Area -->
  <div class="flex-1 flex flex-col min-h-screen">
    <!-- Content -->
    <main class="flex-1 p-8">
      {#if $activeTab === 'diagnostics'}
        <DiagnosticsTab />
      {:else if $activeTab === 'nodes'}
        <NodesTab />
      {:else if $activeTab === 'tests'}
        <TestsTab />
      {/if}
    </main>
  </div>
  
  <!-- Global Modal Container -->
  <Modal />
  
  <!-- Global Notifications -->
  <Notifications />
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