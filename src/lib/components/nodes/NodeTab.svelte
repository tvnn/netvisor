<script lang="ts">
  import { onMount } from 'svelte';
  import { Search } from 'lucide-svelte';
  import { nodes, nodeActions, loading, error } from './store';
  import { nodeGroups, nodeGroupActions } from '../node_groups/store';
  import { getNodeTargetString, type Node } from './types';
  import NodeCard from './NodeCard.svelte';
  import NodeEditor from './NodeEditModal/NodeEditor.svelte';
	import Error from '../common/Error.svelte';
	import Loading from '../common/Loading.svelte';
	import EmptyState from '../common/layout/EmptyState.svelte';
	import SummaryStats from '../common/layout/SummaryStats.svelte';
	import TabHeader from '../common/layout/TabHeader.svelte';
	import SearchField from '../common/layout/SearchField.svelte';
	import DiscoveryStatus from '../discovery/DiscoveryStatus.svelte';
  
  let searchTerm = '';
  let showNodeEditor = false;
  let editingNode: Node | null = null;
  
  $: filteredNodes = $nodes.filter((node: Node) => {
    const searchLower = searchTerm.toLowerCase();
    const targetString = getNodeTargetString(node.target).toLowerCase();
    
    return node.name.toLowerCase().includes(searchLower) ||
          targetString.includes(searchLower) ||
          (node.description && node.description.toLowerCase().includes(searchLower));
  });

  $: groupInfoMap = new Map(
  $nodeGroups.map(group => [
    group.id, 
    {
      name: group.name,
      auto_diagnostic_enabled: group.auto_diagnostic_enabled
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

  function handleDiscovery() {

  }
</script>

<div class="space-y-6">
  <!-- Header -->
   <TabHeader
    title="Nodes"
    subtitle="Manage network endpoints and services"
    buttons={[
      {
        onClick: handleCreateNode,
        cta: "Add Node"
      }
    ]}
    CenterComponent={DiscoveryStatus}
     />

  <!-- Summary stats -->
  <!-- {#if $nodes.length > 0}
    <SummaryStats 
      totalStatLabel="Total Nodes"
      totalStatValue={$nodes.length}
      goodStatLabel="Healthy"
      goodStatValue={$nodes.filter((n: Node) => n.status === 'Healthy').length}
      badStatLabel="Failed"
      badStatValue={$nodes.filter((n: Node) => n.status === 'Failed').length}
      infoStatLabel="Monitored"
      infoStatValue={$nodes.filter((n: Node) => n.monitoring_interval > 0).length}
    />
  {/if}

  <SearchField searchTerm={searchTerm} placeholder="Search nodes by name, IP, or domain..." /> -->

  <Error error={$error} onClear={nodeActions.clearError}/>

  <!-- Loading state -->
  {#if $loading}
    <Loading/>
  {:else if filteredNodes.length === 0}
    <!-- Empty state -->
    <div class="text-center py-12">
      {#if $nodes.length === 0}
        <EmptyState 
          title="No nodes configured yet"
          subtitle=""
          onClick={handleCreateNode}
          cta="Create your first node"/>
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
          groupInfo={node.node_groups ? getGroupInfo(node.node_groups) : []}
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