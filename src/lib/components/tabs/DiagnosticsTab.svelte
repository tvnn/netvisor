<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus, Users, Play, Trash2, Edit } from 'lucide-svelte';
  import { nodeGroups, nodeGroupActions, loading, error } from '../../stores/node-groups';
  import { nodes, nodeActions } from '../../stores/nodes';
  import type { Node } from '../../types/nodes';
  import type { NodeGroup } from '$lib/types/node-groups';
  import NodeGroupEditor from '../modals/NodeGroupEditor.svelte';
	import NodeGroupCard from '../cards/NodeGroupCard.svelte';
  
  let showGroupEditor = false;
  let editingGroup: NodeGroup | null = null;
  
  onMount(() => {
    nodeGroupActions.loadGroups();
    nodeActions.loadNodes()
  });
  
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
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-2xl font-bold text-white">Diagnostic Groups</h2>
      <p class="text-gray-400 mt-1">Manage diagnostic groups</p>
    </div>
    <button
      on:click={handleCreateGroup}
      class="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
    >
      <Plus class="w-4 h-4" />
      Create Diagnostic Group
    </button>
  </div>

  <!-- Summary stats -->
  {#if $nodeGroups.length > 0}
    <div class="bg-gray-800 rounded-lg border border-gray-700 p-4">
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-center">
        <div>
          <div class="text-2xl font-bold text-white">{$nodeGroups.length}</div>
          <div class="text-sm text-gray-400">Total Groups</div>
        </div>
        <div>
          <div class="text-2xl font-bold text-green-400">{$nodeGroups.filter(g => g.auto_diagnostic_enabled).length}</div>
          <div class="text-sm text-gray-400">Auto-Diagnostic</div>
        </div>
        <div>
          <div class="text-2xl font-bold text-purple-400">{$nodeGroups.reduce((sum, g) => sum + g.node_sequence.length, 0)}</div>
          <div class="text-sm text-gray-400">Total Nodes</div>
        </div>
        <div>
          <div class="text-2xl font-bold text-blue-400">0</div>
          <div class="text-sm text-gray-400">Recent Runs</div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Error display -->
  {#if $error}
    <div class="bg-red-600/20 border border-red-600/50 rounded-lg p-4">
      <p class="text-red-300">{$error}</p>
      <button
        on:click={nodeGroupActions.clearError}
        class="text-red-400 hover:text-red-300 text-sm mt-2"
      >
        Dismiss
      </button>
    </div>
  {/if}

  <!-- Loading state -->
  {#if $loading}
    <div class="text-center py-8">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto"></div>
      <p class="text-gray-400 mt-2">Loading node groups...</p>
    </div>
  {:else if $nodeGroups.length === 0}
    <!-- Empty state -->
    <div class="text-center py-12">
      <Users class="w-16 h-16 text-gray-600 mx-auto mb-4" />
      <p class="text-gray-400 text-lg mb-4">No diagnostic groups configured yet</p>
      <p class="text-gray-500 mb-6">Diagnostic groups define test sequences for systematic troubleshooting</p>
      <button
        on:click={handleCreateGroup}
        class="inline-flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
      >
        <Plus class="w-4 h-4" />
        Create your first diagnostic group
      </button>
    </div>
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
<NodeGroupEditor
  isOpen={showGroupEditor}
  group={editingGroup}
  onCreate={handleGroupCreate}
  onUpdate={handleGroupUpdate}
  onClose={handleCloseGroupEditor}
/>