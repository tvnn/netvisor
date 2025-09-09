<script lang="ts">
	import { hosts } from '$lib/features/hosts/store';
	import TabHeader from '$lib/shared/components/layout/TabHeader.svelte';
	import { createHostGroup, deleteHostGroup, hostGroups, updateHostGroup } from '../store';
	import type { HostGroup } from '../types/base';
	import HostGroupCard from './HostGroupCard.svelte';
  	import type { Host } from "$lib/features/hosts/types/base";
	import HostGroupEditModal from './HostGroupEditModal.svelte';
	import SummaryStats from '$lib/shared/components/layout/SummaryStats.svelte';
	import Loading from '$lib/shared/components/feedback/Loading.svelte';
	import EmptyState from '$lib/shared/components/layout/EmptyState.svelte';
	import { loading } from '$lib/shared/stores/feedback';
  
  let showGroupEditor = false;
  let editingGroup: HostGroup | null = null;
    
  function handleCreateGroup() {
    editingGroup = null;
    showGroupEditor = true;
  }
  
  function handleEditGroup(group: HostGroup) {
    editingGroup = group;
    showGroupEditor = true;
  }

  // function handleExecuteDiagnostics(group: HostGroup) {
  //   let data = {
  //     group_id: group.id,
  //     trigger_reason: "Manual"
  //   }
  //   executeDiagnostics(group.id, data);
  // }
  
  function handleDeleteGroup(group: HostGroup) {
    if (confirm(`Are you sure you want to delete "${group.name}"?`)) {
      deleteHostGroup(group.id);
    }
  }
  
  async function handleGroupCreate(data: HostGroup) {
    const result = await createHostGroup(data);
    if (result?.success) {
      showGroupEditor = false;
      editingGroup = null;
    }
  }
  
  async function handleGroupUpdate(data: HostGroup) {
    const result = await updateHostGroup(data);
    if (result?.success) {
      showGroupEditor = false;
      editingGroup = null;
    }
  }
  
  function handleCloseGroupEditor() {
    showGroupEditor = false;
    editingGroup = null;
  }

  function getHosts(hostIds: string[]): Host[] {
    return $hosts.filter(n => n.id in hostIds)
  }
</script>

<div class="space-y-6">
  <TabHeader
    title="Groups"
    subtitle="Create host groups to define logical network groups for visualization"
    buttons={[
      {
        onClick: handleCreateGroup,
        cta: "Create Group"
      }
    ]}
    />

  <!-- {#if $hostGroups.length > 0}
    <SummaryStats 
      totalStatLabel="Total Groups"
      totalStatValue={$hostGroups.length}
      goodStatLabel="Auto-Diagnostic"
      goodStatValue={$hostGroups.filter(g => g.auto_diagnostic_enabled).length}
    />
  {/if} -->

  {#if $hostGroups.length === 0 && !$loading}
    <!-- Empty state -->
    <EmptyState 
      title="No diagnostic groups configured yet"
      subtitle="Diagnostic groups define test sequences for systematic troubleshooting"
      onClick={handleCreateGroup}
      cta="Create your first diagnostic group"/>
  {:else}
    <!-- Host Groups Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each $hostGroups as group (group.id)}
      <HostGroupCard
        {group}
        hosts={getHosts(group.hosts)}
        onEdit={() => handleEditGroup(group)}
        onDelete={() => handleDeleteGroup(group)}
      />
    {/each}
    </div>
  {/if}
</div>

<!-- Modal -->
<HostGroupEditModal
  isOpen={showGroupEditor}
  group={editingGroup}
  onCreate={handleGroupCreate}
  onUpdate={handleGroupUpdate}
  onClose={handleCloseGroupEditor}
/>