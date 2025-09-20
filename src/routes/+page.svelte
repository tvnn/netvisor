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
	import { getServices } from '$lib/features/services/store';
  
  let activeTab = 'hosts';
  let appInitialized = false;

  // Valid tab names for validation
  const validTabs = ['hosts', 'subnets', 'groups', 'topology'];
  
  // Function to get initial tab from URL hash
  function getInitialTab(): string {
    if (typeof window !== "undefined") {
      const hash = window.location.hash.substring(1); // Remove the #
      return validTabs.includes(hash) ? hash : 'hosts';
    }
    return 'hosts';
  }
  
  function handleTabChange(tab: string) {
    if (validTabs.includes(tab)) {
      activeTab = tab;
      
      // Update URL hash without triggering page reload
      if (typeof window !== "undefined") {
        window.location.hash = tab;
      }
    }
  }

  // Function to handle browser navigation (back/forward)
  function handleHashChange() {
    if (typeof window !== "undefined") {
      const hash = window.location.hash.substring(1);
      if (validTabs.includes(hash) && hash !== activeTab) {
        activeTab = hash;
      }
    }
  }

  let registryLoaded = false;
  let servicesLoaded = false;
  let subnetsLoaded = false;

  onMount(async () => {
    // Set initial tab from URL hash
    activeTab = getInitialTab();
    
    // Listen for hash changes (browser back/forward)
    if (typeof window !== "undefined") {
      window.addEventListener('hashchange', handleHashChange);
    }
    // Load initial data
    await getRegistry().then(() => registryLoaded = true);

    await Promise.all([
      getHosts(),
      getDaemons(),
      getHostGroups(),
      getSubnets().then(() => subnetsLoaded = true),
      getActiveDiscoverySessions(),
      getTopology(),
      getServices().then(() => servicesLoaded = true)
    ]);

    setTimeout(() => {
      appInitialized = true;
    }, 50);
  });

  onDestroy(() => {
    stopDiscoveryPolling();

    if (typeof window !== "undefined") {
      window.removeEventListener('hashchange', handleHashChange);
    }
  });

  $: dataReady = registryLoaded && servicesLoaded && subnetsLoaded;
</script>
{#if dataReady}
  <div class="min-h-screen bg-gray-900 text-white flex">
    <!-- Sidebar -->
    <Sidebar {activeTab} onTabChange={handleTabChange} />
    
    <!-- Main Content -->
    <main class="flex-1 overflow-auto">
      <div class="p-8">
        {#if (appInitialized && $loading)}
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
{/if}

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