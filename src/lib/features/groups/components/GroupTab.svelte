<script lang="ts">
	import TabHeader from '$lib/shared/components/layout/TabHeader.svelte';
	import { createGroup, deleteGroup, getGroups, groups, updateGroup } from '../store';
	import type { Group } from '../types/base';
	import GroupCard from './GroupCard.svelte';
	import GroupEditModal from './GroupEditModal/GroupEditModal.svelte';
	import EmptyState from '$lib/shared/components/layout/EmptyState.svelte';
	import { loadData } from '$lib/shared/utils/dataLoader';
	import { getServices } from '$lib/features/services/store';
	import Loading from '$lib/shared/components/feedback/Loading.svelte';

  const loading = loadData([getGroups, getServices]);
  
  let showGroupEditor = false;
  let editingGroup: Group | null = null;
    
  function handleCreateGroup() {
    editingGroup = null;
    showGroupEditor = true;
  }
  
  function handleEditGroup(group: Group) {
    editingGroup = group;
    showGroupEditor = true;
  }
  
  function handleDeleteGroup(group: Group) {
    if (confirm(`Are you sure you want to delete "${group.name}"?`)) {
      deleteGroup(group.id);
    }
  }
  
  async function handleGroupCreate(data: Group) {
    const result = await createGroup(data);
    if (result?.success) {
      showGroupEditor = false;
      editingGroup = null;
    }
  }
  
  async function handleGroupUpdate(id: string, data: Group) {
    const result = await updateGroup(data);
    if (result?.success) {
      showGroupEditor = false;
      editingGroup = null;
    }
  }
  
  function handleCloseGroupEditor() {
    showGroupEditor = false;
    editingGroup = null;
  }
</script>

<div class="space-y-6">
  <TabHeader
    title="Groups"
    subtitle="Create groups to define logical network groups for visualization"
    buttons={[
      {
        onClick: handleCreateGroup,
        cta: "Create Group"
      }
    ]}
    />

  {#if $loading}
      <Loading/>
  {:else if $groups.length === 0}
    <!-- Empty state -->
    <EmptyState 
      title="No groups configured yet"
      subtitle="Groups define clusters or paths of nodes for visualization"
      onClick={handleCreateGroup}
      cta="Create your first group"/>
  {:else}
    <!-- Groups Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each $groups as group (group.id)}
      <GroupCard
        {group}
        onEdit={() => handleEditGroup(group)}
        onDelete={() => handleDeleteGroup(group)}
      />
    {/each}
    </div>
  {/if}
</div>

<!-- Modal -->
<GroupEditModal
  isOpen={showGroupEditor}
  group={editingGroup}
  onCreate={handleGroupCreate}
  onUpdate={handleGroupUpdate}
  onClose={handleCloseGroupEditor}
/>