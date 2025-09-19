<script lang="ts">
	import { hostGroups } from '$lib/features/host_groups/store';
	import { consolidateHosts, createHost, deleteHost, getHostTargetString, hosts, updateHost } from '../store';
  import HostCard from './HostCard.svelte';
  import type { Host } from '../types/base';
	import TabHeader from '$lib/shared/components/layout/TabHeader.svelte';
	import Loading from '$lib/shared/components/feedback/Loading.svelte';
	import EmptyState from '$lib/shared/components/layout/EmptyState.svelte';
	import { loading } from '$lib/shared/stores/feedback';
	import { getHostDaemon } from '$lib/features/daemons/store';
	import type { Daemon } from '$lib/features/daemons/types/base';
	import { initiateDiscovery, sessions } from '$lib/features/discovery/store';
	import HostEditor from './HostEditModal/HostEditor.svelte';
	import HostConsolidationModal from './HostConsolidationModal.svelte';

  let searchTerm = '';
  let showHostEditor = false;
  let editingHost: Host | null = null;

  let otherHost: Host | null = null;
  let showHostConsolidationModal = false;

  $: discoveryIsRunning = $sessions.size > 0;
  
  $: filteredHosts = $hosts.filter((host: Host) => {
    const searchLower = searchTerm.toLowerCase();
    const targetString = getHostTargetString(host.target).toLowerCase();
    
    return host.name.toLowerCase().includes(searchLower) ||
          targetString.includes(searchLower) ||
          (host.description && host.description.toLowerCase().includes(searchLower));
  });

  $: groupInfoMap = new Map(
  $hostGroups.map(group => [
    group.id, 
    {
      name: group.name,
    }
  ])
);
  
  // Helper function to get group info from IDs
  function getGroupInfo(groupIds: string[]) {
  return groupIds.map(id => 
    groupInfoMap.get(id) || {
      name: id.slice(0, 8) + '...',
      auto_diagnostic_enabled: false
    }
  );
  }
  
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

  function handleConvert(host: Host) {
    otherHost = host;
    showHostConsolidationModal = true;
  }
  
  function handleDeleteHost(host: Host) {
    if (confirm(`Are you sure you want to delete "${host.name}"?`)) {
      deleteHost(host.id);
    }
  }
  
  async function handleHostCreate(data: Host) {
    const result = await createHost(data);
    if (result?.success) {
      showHostEditor = false;
      editingHost = null;
    }
  }
  
  async function handleHostUpdate(id: string, data: any) {
    const result = await updateHost(data);
    if (result?.success) {
      showHostEditor = false;
      editingHost = null;
    }
  }

  async function handleHostConvert(destination_host_id: string, other_host_id: string) {
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
  {#if filteredHosts.length === 0 && !$loading}
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
          groupInfo={host.groups ? getGroupInfo(host.groups) : []}
          discoveryIsRunning={discoveryIsRunning}
          onEdit={handleEditHost}
          onDelete={handleDeleteHost}
          onDiscovery={handleRunDiscovery}
          onConvert={handleConvert}
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
  onConvert={handleHostConvert}
  onClose={() => (showHostConsolidationModal = false)}
/>