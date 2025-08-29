<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus, Users, Play, Trash2, Edit } from 'lucide-svelte';
  import { nodes, nodeActions } from '../nodes/store';
  import type { Node } from '../nodes/types';
  import type { NodeGroup } from '$lib/components/node_groups/types';
	import { error, loading, nodeGroupActions, nodeGroups } from './store';
	import NodeGroupCard from './NodeGroupCard.svelte';
	import NodeGroupEditModal from './NodeGroupEditModal.svelte';
  import Error from '../common/Error.svelte';
	import Loading from '../common/Loading.svelte';
	import EmptyState from '../common/EmptyState.svelte';
	import SummaryStats from '../common/SummaryStats.svelte';
	import TabHeader from '../common/TabHeader.svelte';
  
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
    nodeGroupActions.executeNodeGroupDiagnostics(group.id, data);
  }
  
  function handleDeleteGroup(group: NodeGroup) {
    if (confirm(`Are you sure you want to delete "${group.name}"?`)) {
      nodeGroupActions.deleteGroup(group.id);
    }
  }
  
  async function handleGroupCreate(data: any) {
    const result = await nodeGroupActions.createGroup(data);
    if (result) {
      showGroupEditor = false;
      editingGroup = null;
    }
  }
  
  async function handleGroupUpdate(id: string, data: any) {
    const result = await nodeGroupActions.updateGroup(id, data);
    if (result) {
      showGroupEditor = false;
      editingGroup = null;
    }
  }
  
  function handleCloseGroupEditor() {
    showGroupEditor = false;
    editingGroup = null;
  }

  function getNodes(nodeIds: string[]): Node[] {
    return nodeIds.map(id => $nodes.find(n => n.id === id)).filter(Boolean) as Node[];
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

  <Error error={$error} onClear={nodeGroupActions.clearError}/>

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