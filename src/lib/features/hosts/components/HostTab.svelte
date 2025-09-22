<script lang="ts">
  import HostCard from './HostCard.svelte';
  import type { Host, HostWithServicesRequest } from '../types/base';
	import TabHeader from '$lib/shared/components/layout/TabHeader.svelte';
	import Loading from '$lib/shared/components/feedback/Loading.svelte';
	import EmptyState from '$lib/shared/components/layout/EmptyState.svelte';
	import { getDaemons, getHostDaemon } from '$lib/features/daemons/store';
	import type { Daemon } from '$lib/features/daemons/types/base';
	import { getActiveDiscoverySessions, initiateDiscovery, sessions } from '$lib/features/discovery/store';
	import HostEditor from './HostEditModal/HostEditor.svelte';
	import HostConsolidationModal from './HostConsolidationModal.svelte';
	import { consolidateHosts, createHost, deleteHost, getHosts, getHostTargetString, hosts, updateHost } from '../store';
	import { getGroups, groups } from '$lib/features/groups/store';
	import { loadData } from '$lib/shared/utils/dataLoader';
	import { getServices, getServicesForHost, services } from '$lib/features/services/store';
	import { getSubnets, subnets } from '$lib/features/subnets/store';
	import { stringify } from 'uuid';
	import { get } from 'svelte/store';
	import { watchStores } from '$lib/shared/utils/storeWatcher';

  const loading = loadData([
    getHosts, getGroups, getServices, getSubnets, getDaemons, getActiveDiscoverySessions
  ]);

  let searchTerm = '';
  let showHostEditor = false;
  let editingHost: Host | null = null;

  let otherHost: Host | null = null;
  let showHostConsolidationModal = false;

  $: discoveryIsRunning = $sessions.size > 0;
  
  $: filteredHosts = $hosts.filter((host: Host) => {
    const searchLower = searchTerm.toLowerCase();
    const targetString = getHostTargetString(host);
    
    return host.name.toLowerCase().includes(searchLower) ||
          targetString.includes(searchLower) ||
          (host.description && host.description.toLowerCase().includes(searchLower));
  });

  $: hostGroups = new Map(
    $hosts.map(host => {
      const serviceGroups = getServicesForHost(host.id).flatMap(s => s.groups);
      const foundGroups = serviceGroups
        .map(group_id => $groups.find(g => g.id === group_id))
        .filter(group => group !== undefined);
      
      return [host.id, foundGroups];
    })
  );
      
  function handleCreateHost() {
    editingHost = null;
    showHostEditor = true;
  }
  
  function handleEditHost(host: Host) {
    editingHost = host;
    showHostEditor = true;
  }

  function handleRunDiscovery(daemon: Daemon) {
    initiateDiscovery({daemon_id: daemon.id})
  }

  function handleStartConsolidate(host: Host) {
    otherHost = host;
    showHostConsolidationModal = true;
  }
  
  function handleDeleteHost(host: Host) {
    if (confirm(`Are you sure you want to delete "${host.name}"?`)) {
      deleteHost(host.id);
    }
  }
  
  async function handleHostCreate(data: HostWithServicesRequest) {
    const result = await createHost(data);
    if (result?.success) {
      showHostEditor = false;
      editingHost = null;
    }
  }
  
  async function handleHostUpdate(data: HostWithServicesRequest) {
    const result = await updateHost(data);
    if (result?.success) {
      showHostEditor = false;
      editingHost = null;
    }
  }

  async function handleConsolidateHosts(destination_host_id: string, other_host_id: string) {
    const result = await consolidateHosts(destination_host_id, other_host_id);
    if (result?.success) {
      showHostConsolidationModal = false;
      otherHost = null;
    }
  }
  
  function handleCloseHostEditor() {
    showHostEditor = false;
    editingHost = null;
  }
</script>

<div class="space-y-6">
  <!-- Header -->
   <TabHeader
    title="Hosts"
    subtitle="Manage hosts on the network"
    buttons={[
      {
        onClick: handleCreateHost,
        cta: "Create Host"
      }
    ]}
     />


  <!-- Loading state -->
  {#if $loading}
      <Loading/>
  {:else if filteredHosts.length === 0}
    <!-- Empty state -->
    <div class="text-center py-12">
      {#if $hosts.length === 0}
        <EmptyState 
          title="No hosts configured yet"
          subtitle=""
          onClick={handleCreateHost}
          cta="Create your first host"/>
      {:else}
        <p class="text-gray-400 text-lg">No hosts match your search</p>
      {/if}
    </div>
  {:else}
    <!-- Hosts grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each filteredHosts as host (host.id)}
        <HostCard
          {host}
          daemon={getHostDaemon(host.id)}
          hostGroups={hostGroups.get(host.id)}
          discoveryIsRunning={discoveryIsRunning}
          onEdit={handleEditHost}
          onDelete={handleDeleteHost}
          onDiscovery={handleRunDiscovery}
          onConsolidate={handleStartConsolidate}
        />
      {/each}
    </div>
  {/if}
</div>

<HostEditor
  isOpen={showHostEditor}
  host={editingHost}
  onCreate={handleHostCreate}
  onUpdate={handleHostUpdate}
  onClose={handleCloseHostEditor}
/>

<HostConsolidationModal
  isOpen={showHostConsolidationModal}
  otherHost={otherHost}
  onConsolidate={handleConsolidateHosts}
  onClose={() => (showHostConsolidationModal = false)}
/>