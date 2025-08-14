<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus, Search } from 'lucide-svelte';
  import { nodes, nodeActions, loading, error } from '../../stores/nodes';
  import { nodeGroups, nodeGroupActions } from '../../stores/node-groups';
  import type { Node } from '../../types/nodes';
  import NodeCard from '../cards/NodeCard.svelte';
  import NodeEditor from '../modals/NodeEditor/NodeEditor.svelte';
  
  let searchTerm = '';
  let showNodeEditor = false;
  let editingNode: Node | null = null;
  
  $: filteredNodes = $nodes.filter((node: Node) => 
    node.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
    (node.ip && node.ip.includes(searchTerm)) ||
    (node.domain && node.domain.toLowerCase().includes(searchTerm.toLowerCase()))
  );
  
  // Helper function to get group names from IDs
  function getGroupNames(groupIds: string[]): string[] {
    return groupIds.map(id => {
      const group = $nodeGroups.find(g => g.id === id);
      return group ? group.name : id.slice(0, 8) + '...';
    });
  }
  
  onMount(() => {
    nodeActions.loadNodes();
    nodeGroupActions.loadGroups();
  });
  
  function handleCreateNode() {
    editingNode = null;
    showNodeEditor = true;
  }
  
  function handleEditNode(node: Node) {
    editingNode = node;
    showNodeEditor = true;
  }
  
  function handleDeleteNode(node: Node) {
    if (confirm(`Are you sure you want to delete "${node.name}"?`)) {
      nodeActions.deleteNode(node.id);
    }
  }
  
  async function handleNodeCreate(data: any) {
    const result = await nodeActions.createNode(data);
    if (result) {
      showNodeEditor = false;
      editingNode = null;
    }
  }
  
  async function handleNodeUpdate(id: string, data: any) {
    const result = await nodeActions.updateNode(id, data);
    if (result) {
      showNodeEditor = false;
      editingNode = null;
    }
  }
  
  function handleCloseNodeEditor() {
    showNodeEditor = false;
    editingNode = null;
  }
</script>

<div class="space-y-6">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-2xl font-bold text-white">Network Nodes</h2>
      <p class="text-gray-400 mt-1">Manage network endpoints and services</p>
    </div>
    <button
      on:click={handleCreateNode}
      class="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
    >
      <Plus class="w-4 h-4" />
      Add Node
    </button>
  </div>

  <!-- Summary stats -->
  {#if $nodes.length > 0}
    <div class="bg-gray-800 rounded-lg border border-gray-700 p-4">
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-center">
        <div>
          <div class="text-2xl font-bold text-white">{$nodes.length}</div>
          <div class="text-sm text-gray-400">Total Nodes</div>
        </div>
        <div>
          <div class="text-2xl font-bold text-green-400">{$nodes.filter((n: Node) => n.current_status === 'Healthy').length}</div>
          <div class="text-sm text-gray-400">Healthy</div>
        </div>
        <div>
          <div class="text-2xl font-bold text-red-400">{$nodes.filter((n: Node) => n.current_status === 'Failed').length}</div>
          <div class="text-sm text-gray-400">Failed</div>
        </div>
        <div>
          <div class="text-2xl font-bold text-blue-400">{$nodes.filter((n: Node) => n.monitoring_enabled).length}</div>
          <div class="text-sm text-gray-400">Monitored</div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Search -->
  <div class="relative">
    <Search class="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4" />
    <input
      bind:value={searchTerm}
      type="text"
      placeholder="Search nodes by name, IP, or domain..."
      class="w-full pl-10 pr-4 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
    />
  </div>

  <!-- Error display -->
  {#if $error}
    <div class="bg-red-600/20 border border-red-600/50 rounded-lg p-4">
      <p class="text-red-300">{$error}</p>
      <button
        on:click={nodeActions.clearError}
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
      <p class="text-gray-400 mt-2">Loading nodes...</p>
    </div>
  {:else if filteredNodes.length === 0}
    <!-- Empty state -->
    <div class="text-center py-12">
      {#if $nodes.length === 0}
        <p class="text-gray-400 text-lg mb-4">No nodes configured yet</p>
        <button
          on:click={handleCreateNode}
          class="inline-flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
        >
          <Plus class="w-4 h-4" />
          Create your first node
        </button>
      {:else}
        <p class="text-gray-400 text-lg">No nodes match your search</p>
      {/if}
    </div>
  {:else}
    <!-- Nodes grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each filteredNodes as node (node.id)}
        <NodeCard
          {node}
          groupNames={getGroupNames(node.node_groups)}
          onEdit={handleEditNode}
          onDelete={handleDeleteNode}
        />
      {/each}
    </div>
  {/if}
</div>

<NodeEditor
  isOpen={showNodeEditor}
  node={editingNode}
  onCreate={handleNodeCreate}
  onUpdate={handleNodeUpdate}
  onClose={handleCloseNodeEditor}
/>