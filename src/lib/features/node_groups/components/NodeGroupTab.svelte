<script lang="ts">
	import { executeDiagnostics } from '$lib/features/diagnostics/store';
	import { nodes } from '$lib/features/nodes/store';
	import TabHeader from '$lib/shared/components/layout/TabHeader.svelte';
	import { createNodeGroup, deleteNodeGroup, nodeGroups, updateNodeGroup } from '../store';
	import type { NodeGroup } from '../types/base';
	import NodeGroupCard from './NodeGroupCard.svelte';
  	import type { Node } from "$lib/features/nodes/types/base";
	import NodeGroupEditModal from './NodeGroupEditModal.svelte';
	import SummaryStats from '$lib/shared/components/layout/SummaryStats.svelte';
	import ErrorBanner from '$lib/shared/components/feedback/ErrorBanner.svelte';
	import Loading from '$lib/shared/components/feedback/Loading.svelte';
	import EmptyState from '$lib/shared/components/layout/EmptyState.svelte';
	import { loading } from '$lib/shared/stores/feedback';
  
  let showGroupEditor = false;
  let editingGroup: NodeGroup | null = null;
    
  function handleCreateGroup() {
    editingGroup = null;
    showGroupEditor = true;
  }
  
  function handleEditGroup(group: NodeGroup) {
    editingGroup = group;
    showGroupEditor = true;
  }

  function handleExecuteDiagnostics(group: NodeGroup) {
    let data = {
      group_id: group.id,
      trigger_reason: "Manual"
    }
    executeDiagnostics(group.id, data);
  }
  
  function handleDeleteGroup(group: NodeGroup) {
    if (confirm(`Are you sure you want to delete "${group.name}"?`)) {
      deleteNodeGroup(group.id);
    }
  }
  
  async function handleGroupCreate(data: NodeGroup) {
    const result = await createNodeGroup(data);
    if (result?.success) {
      showGroupEditor = false;
      editingGroup = null;
    }
  }
  
  async function handleGroupUpdate(data: NodeGroup) {
    const result = await updateNodeGroup(data);
    if (result?.success) {
      showGroupEditor = false;
      editingGroup = null;
    }
  }
  
  function handleCloseGroupEditor() {
    showGroupEditor = false;
    editingGroup = null;
  }

  function getNodes(nodeIds: string[]): Node[] {
    return $nodes.filter(n => n.id in nodeIds)
  }
</script>

<div class="space-y-6">
  <TabHeader
    title="Groups"
    subtitle="Create node groups to define logical network paths for diagnostics"
    buttons={[
      {
        onClick: handleCreateGroup,
        cta: "Create Group"
      }
    ]}
    />

  {#if $nodeGroups.length > 0}
    <SummaryStats 
      totalStatLabel="Total Groups"
      totalStatValue={$nodeGroups.length}
      goodStatLabel="Auto-Diagnostic"
      goodStatValue={$nodeGroups.filter(g => g.auto_diagnostic_enabled).length}
    />
  {/if}

  <ErrorBanner/>

  {#if $loading}
    <Loading/>
  {:else if $nodeGroups.length === 0}
    <!-- Empty state -->
    <EmptyState 
      title="No diagnostic groups configured yet"
      subtitle="Diagnostic groups define test sequences for systematic troubleshooting"
      onClick={handleCreateGroup}
      cta="Create your first diagnostic group"/>
  {:else}
    <!-- Node Groups Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each $nodeGroups as group (group.id)}
      <NodeGroupCard
        {group}
        nodes={getNodes(group.node_sequence)}
        onEdit={() => handleEditGroup(group)}
        onDelete={() => handleDeleteGroup(group)}
        onExecute={() => handleExecuteDiagnostics(group)}
      />
    {/each}
    </div>
  {/if}
</div>

<!-- Modal -->
<NodeGroupEditModal
  isOpen={showGroupEditor}
  group={editingGroup}
  onCreate={handleGroupCreate}
  onUpdate={handleGroupUpdate}
  onClose={handleCloseGroupEditor}
/>