<script lang="ts">
	import { getDaemons } from '$lib/features/daemons/store';
	import { getActiveDiscoverySessions, stopDiscoveryPolling } from '$lib/features/discovery/store';
	import HostGroupTab from '$lib/features/host_groups/components/HostGroupTab.svelte';
	import { getHostGroups } from '$lib/features/host_groups/store';
	import HostTab from '$lib/features/hosts/components/HostTab.svelte';
  import TopologyTab from '$lib/features/topology/components/TopologyTab.svelte';
	import { getHosts } from '$lib/features/hosts/store';
	import SubnetTab from '$lib/features/subnets/components/SubnetTab.svelte';
	import { getSubnets } from '$lib/features/subnets/store';
	import Loading from '$lib/shared/components/feedback/Loading.svelte';
	import Toast from '$lib/shared/components/feedback/Toast.svelte';
	import Sidebar from '$lib/shared/components/layout/Sidebar.svelte';
	import { loading } from '$lib/shared/stores/feedback';
	import { getRegistry } from '$lib/shared/stores/registry';
	import { onDestroy, onMount } from 'svelte';
	import { getTopology } from '$lib/features/topology/store';
  
  let activeTab = 'hosts';
  let appInitialized = false;
  
  function handleTabChange(tab: string) {
    activeTab = tab;
  }

  onMount(async () => {
    // Load initial data
    await Promise.all([
      getRegistry(),
      getHosts(),
      getDaemons(),
      getHostGroups(),
      getSubnets(),
      getActiveDiscoverySessions(),
      getTopology()
    ]);

    setTimeout(() => {
      appInitialized = true;
    }, 50);
  });

  onDestroy(() => {
    stopDiscoveryPolling();
  });
</script>

<div class="min-h-screen bg-gray-900 text-white flex">
  <!-- Sidebar -->
  <Sidebar {activeTab} onTabChange={handleTabChange} />
  
  <!-- Main Content -->
  <main class="flex-1 overflow-auto">
    <div class="p-8">
      {#if appInitialized && $loading}
        <Loading />
      {:else if activeTab === 'hosts'}
        <HostTab />
      {:else if activeTab === 'subnets'}
        <SubnetTab />
      {:else if activeTab === 'groups'}
        <HostGroupTab />
      {:else if activeTab === 'topology'}
        <TopologyTab />
      {/if}
    </div>

    <Toast />

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