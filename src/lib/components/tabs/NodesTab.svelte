<!-- Update src/lib/components/tabs/NodesTab.svelte -->
<script lang="ts">
  import { Plus, Server, FolderOpen, Folder } from 'lucide-svelte';
  import { nodes, nodeActions } from '../../stores/nodes';
  import { modalActions, notificationActions } from '../../stores/ui';
  import NodeEditor from '../modals/NodeEditor.svelte';
  import ConfirmDialog from '../modals/ConfirmDialog.svelte';
  import Card from '../shared/Card.svelte';
    
  function createNode() {
    modalActions.open(NodeEditor, { mode: 'create' }, 'Create Network Node');
  }
  
  function editNode(node: any) {
    modalActions.open(NodeEditor, { node }, 'Edit Network Node');
    modalActions.open(NodeEditor, { 
      mode: 'edit', 
      node: { ...node } 
    }, `Edit ${node.name}`);
  }
  
  async function duplicateNode(nodeId: string) {
    try {
      await nodeActions.duplicate(nodeId);
    } catch (error) {
      console.error('Failed to duplicate node:', error);
    }
  }
  
  async function deleteNode(node: any) {
    modalActions.open(ConfirmDialog, {
      title: 'Delete Node',
      message: `Are you sure you want to delete "${node.name}"? This action cannot be undone.`,
      confirmText: 'Delete',
      cancelText: 'Cancel',
      danger: true,
      onConfirm: async () => {
        try {
          await nodeActions.delete(node.id);
          notificationActions.success(`Deleted test: ${node.name}`);
        } catch (error) {
          notificationActions.error('Failed to delete test');
          console.error('Failed to delete test:', error);
        }
      }
    }, 'Confirm Deletion');
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
      on:click={createNode}
      class="btn-primary flex items-center gap-2"
    >
      <Plus class="w-4 h-4" />
      Add Node
    </button>
  </div>

  <!-- Nodes organized by folder -->
   <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
    {#each $nodes as node}
      <Card
        title={node.name}
        metadata={[
          ...(node.domain ? [{ label: 'Domain', value: node.domain }] : []),
          ...(node.ip ? [{ label: 'IP', value: node.ip }] : []),
          ...(node.defaultPort ? [{ label: 'Port', value: node.defaultPort.toString() }] : [])
        ]}
        description={node.description || ''}
        onEdit={() => editNode(node)}
        onCopy={() => duplicateNode(node.id)}
        onDelete={() => deleteNode(node)}
      />
    {/each}
   </div>

  <!-- Empty state -->
  {#if $nodes.length === 0}
    <div class="text-center py-12">
      <Server class="w-16 h-16 mx-auto text-gray-600 mb-4" />
      <h3 class="text-xl font-semibold text-gray-300 mb-2">No Network Nodes</h3>
      <p class="text-gray-400 mb-6 max-w-md mx-auto">
        Create your first network node
      </p>
      <button
        on:click={createNode}
        class="flex items-center gap-2 px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors mx-auto"
      >
        <Plus class="w-4 h-4" />
        Create Network Node
      </button>
    </div>
  {/if}
</div>