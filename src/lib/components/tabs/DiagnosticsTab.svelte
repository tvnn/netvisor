<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus, Users, Play, Settings, Trash2, Edit } from 'lucide-svelte';
  import { nodeGroups, nodeGroupActions, loading, error } from '../../stores/node-groups';
  import { nodes } from '../../stores/nodes';
  import type { NodeGroup } from '../../stores/node-groups';
  import NodeGroupEditor from '../modals/NodeGroupEditor.svelte';
  
  let showGroupEditor = false;
  let editingGroup: NodeGroup | null = null;
  
  onMount(() => {
    nodeGroupActions.loadGroups();
  });
  
  function handleCreateGroup() {
    editingGroup = null;
    showGroupEditor = true;
  }
  
  function handleEditGroup(group: NodeGroup) {
    editingGroup = group;
    showGroupEditor = true;
  }
  
  function handleDeleteGroup(group: NodeGroup) {
    if (confirm(`Are you sure you want to delete "${group.name}"?`)) {
      nodeGroupActions.deleteGroup(group.id);
    }
  }
  
  // Updated to handle function props instead of events
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
  
  function getNodeNames(nodeIds: string[]): string[] {
    return nodeIds.map(id => {
      const node = $nodes.find(n => n.id === id);
      return node ? node.name : `Node ${id.slice(0, 8)}...`;
    });
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
        <div class="bg-gray-800 rounded-lg border border-gray-700 p-4 hover:border-gray-600 transition-colors">
          <!-- Header -->
          <div class="flex items-start justify-between mb-3">
            <div class="flex items-center gap-2">
              <Users class="w-5 h-5 text-purple-400" />
              <div>
                <h3 class="font-semibold text-white">{group.name}</h3>
                <p class="text-sm text-gray-400">{group.node_sequence.length} nodes</p>
              </div>
            </div>
            
            <!-- Auto-diagnostic indicator -->
            {#if group.auto_diagnostic_enabled}
              <div class="flex items-center gap-1">
                <div class="w-2 h-2 rounded-full bg-green-500"></div>
                <span class="text-xs text-green-400">Auto</span>
              </div>
            {:else}
              <div class="flex items-center gap-1">
                <div class="w-2 h-2 rounded-full bg-gray-500"></div>
                <span class="text-xs text-gray-500">Manual</span>
              </div>
            {/if}
          </div>

          <!-- Description -->
          {#if group.description}
            <p class="text-sm text-gray-300 mb-3">{group.description}</p>
          {/if}

          <!-- Node sequence -->
          <div class="mb-4">
            <div class="text-xs text-gray-400 mb-2">Diagnostic Sequence:</div>
            {#if group.node_sequence.length > 0}
              <div class="space-y-1">
                {#each getNodeNames(group.node_sequence).slice(0, 3) as nodeName, index}
                  <div class="flex items-center gap-2 text-sm">
                    <span class="text-gray-500">{index + 1}.</span>
                    <span class="text-gray-300">{nodeName}</span>
                  </div>
                {/each}
                {#if group.node_sequence.length > 3}
                  <div class="text-xs text-gray-500 ml-4">
                    +{group.node_sequence.length - 3} more nodes...
                  </div>
                {/if}
              </div>
            {:else}
              <span class="text-xs text-gray-500">No nodes in sequence</span>
            {/if}
          </div>

          <!-- Actions -->
          <div class="flex items-center justify-between pt-3 border-t border-gray-700">
            <div class="flex items-center gap-2">
              <button
                class="p-1 text-gray-400 hover:text-white transition-colors"
                title="Run diagnostic"
                disabled
              >
                <Play class="w-4 h-4" />
              </button>
              
              <button
                on:click={() => handleEditGroup(group)}
                class="p-1 text-gray-400 hover:text-white transition-colors"
                title="Edit group"
              >
                <Edit class="w-4 h-4" />
              </button>
            </div>
            
            <button
              on:click={() => handleDeleteGroup(group)}
              class="p-1 text-red-400 hover:text-red-300 transition-colors"
              title="Delete group"
            >
              <Trash2 class="w-4 h-4" />
            </button>
          </div>
        </div>
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