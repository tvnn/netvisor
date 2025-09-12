<script lang="ts">
  import { onMount } from 'svelte';
  import { writable } from 'svelte/store';
  import { 
    SvelteFlow, 
    Controls, 
    Background, 
    MiniMap,
    BackgroundVariant,
    MarkerType
  } from '@xyflow/svelte';
  import type { Node, Edge } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
  import { topology } from '../store';
  import { graph_edge_types } from '$lib/shared/stores/registry';
  import ELK from 'elkjs/lib/elk.bundled.js';
  import { createIconComponent } from '$lib/shared/utils/styling';
  import { pushError } from '$lib/shared/stores/feedback';
  
  // Import custom node components
  import SubnetNode from './SubnetNode.svelte';
  import HostNode from './HostNode.svelte';

  // Define node types
  const nodeTypes = {
    subnet: SubnetNode,
    host: HostNode
  };

  // Stores
  let nodes = writable<Node[]>([]);
  let edges = writable<Edge[]>([]);
  let selectedNodeId: string | null = null;

  // ELK layout instance
  const elk = new ELK();

  onMount(async () => {
    try {
      await loadTopologyData();
    } catch (err) {
      pushError('Failed to load network topology');
    }
  });

  async function loadTopologyData() {    
    try {
      if ($topology?.nodes && $topology?.edges) {
        const flowNodes: Node[] = $topology.nodes.map((node: TopologyNodeData): Node => {
          // Determine node type based on node_type
          const nodeType = node.node_type === 'SubnetNode' ? 'subnet' : 'host';
          
          return {
            id: node.id,
            type: nodeType,
            position: { x: node.position.x, y: node.position.y },
            parentId: node.parent_id || undefined, // This is the key property
            extent: nodeType === 'host' && node.parent_id ? 'parent' : undefined, // Constrains child to parent
            width: node.size.x,
            height: node.size.y,
            data: {
              label: node.label,
              nodeType: node.node_type,
              color: node.color,
              icon: node.icon,
              parentId: node.parent_id,
              width: node.size.x,
              height: node.size.y,
              IconComponent: createIconComponent(node.icon),
              // Additional data that might be useful for display
              // ip: node.ip || null,
              // cidr: node.cidr || null,
              // status: node.status || 'unknown'
            },
          };
        });

        const flowEdges: Edge[] = $topology.edges.map(([sourceIdx, targetIdx, edgeData]: [number, number, TopologyEdgeData], index: number): Edge => {
          const edgeColor = graph_edge_types.getColorString(edgeData.edge_type);
          const edgeLabel = graph_edge_types.getDisplay(edgeData.edge_type);
          
          return {
            id: `edge-${index}`,
            source: edgeData.source,
            target: edgeData.target,
            type: 'smoothstep',
            style: `stroke: ${edgeColor}; stroke-width: 2px;`,
            markerEnd: {
              type: MarkerType.ArrowClosed,
              color: edgeColor
            },
            data: {
              edgeType: edgeData.edge_type
            }
          };
        });

        // Apply ELK layout
        const layoutedGraph = await applyELKLayout(flowNodes, flowEdges);
        
        nodes.set(layoutedGraph.nodes);
        edges.set(layoutedGraph.edges);
      }
    } catch (err) {
      pushError('Failed to parse topology data');
    }
  }

  async function applyELKLayout(flowNodes: Node[], flowEdges: Edge[]) {
    // Separate subnet nodes from host nodes
    const subnetNodes = flowNodes.filter(node => node.type === 'subnet');
    const hostNodes = flowNodes.filter(node => node.type === 'host');
    
    // const elkGraph = {
    //   id: "root",
    //   layoutOptions: {
    //     'elk.algorithm': 'force',
    //     'elk.spacing.nodeNode': '100', // Much more spacing
    //     'elk.padding': '[top=50,left=50,bottom=50,right=50]'
    //   },
    //   children: [
    //     // Subnet nodes as containers
    //     ...subnetNodes.map(subnet => ({
    //       id: subnet.id,
    //       layoutOptions: {
    //         'elk.algorithm': 'force',
    //         'elk.spacing.nodeNode': '50', // Good spacing within subnets
    //         'elk.padding': '[top=40,left=40,bottom=40,right=40]',
    //         'elk.force.repulsivePower': '2.0' // Push nodes apart
    //       },
    //       // Host nodes that belong to this subnet
    //       children: hostNodes
    //         .filter(host => host.data.parentId === subnet.id)
    //         .map(host => ({
    //           id: host.id,
    //           width: 140, // Default width for hosts
    //           height: 100 // Default height for hosts
    //         }))
    //     })),
    //     // Standalone nodes (no parent)
    //     ...hostNodes
    //       .filter(host => !host.data.parentId)
    //       .map(host => ({
    //         id: host.id,
    //       }))
    //   ],
    //   edges: flowEdges.map(edge => ({
    //     id: edge.id,
    //     sources: [edge.source],
    //     targets: [edge.target]
    //   }))
    // };

    // const layoutedGraph = await elk.layout(elkGraph);
    
    // const layoutedNodes = flowNodes.map(node => {
    //   // Find the node in the layout result
    //   let elkNode = layoutedGraph.children?.find(n => n.id === node.id);
      
    //   // If not found at top level, look in subnet containers
    //   if (!elkNode) {
    //     for (const container of layoutedGraph.children || []) {
    //       const foundNode = container.children?.find(n => n.id === node.id);
    //       if (foundNode) {
    //         elkNode = {
    //           id: foundNode.id,
    //           width: foundNode.width || 140,
    //           height: foundNode.height || 100,
    //           x: (foundNode.x || 0) + (container.x || 0),
    //           y: (foundNode.y || 0) + (container.y || 0)
    //         };
    //         break;
    //       }
    //     }
    //   }
      
    //   if (elkNode) {
    //     return {
    //       ...node,
    //       position: { x: elkNode.x || 0, y: elkNode.y || 0 }
    //     };
    //   }
    //   return node;
    // });

    return {
      nodes: flowNodes,
      edges: flowEdges
    };
  }

  function refreshTopology() {
    loadTopologyData();
  }

  async function reapplyLayout() {
    if ($nodes.length > 0 && $edges.length > 0) {
      const layoutedGraph = await applyELKLayout($nodes, $edges);
      nodes.set(layoutedGraph.nodes);
      edges.set(layoutedGraph.edges);
    }
  }

  // Event handlers
  function onNodeClick({ node, event }: { node: Node; event: MouseEvent | TouchEvent }) {
    selectedNodeId = node.id;
    console.log('Node clicked:', node);
  }

  function onEdgeClick({ edge, event }: { edge: Edge; event: MouseEvent }) {
    console.log('Edge clicked:', edge);
  }

  function resetSelection() {
    selectedNodeId = null;
  }

  // Reactive statements
  $: selectedNode = selectedNodeId ? $nodes.find(n => n.id === selectedNodeId) : null;
  $: graphStats = {
    nodes: $nodes.length,
    edges: $edges.length,
    generated_at: new Date().toLocaleTimeString()
  };
</script>

<!-- Controls -->
<div class="mb-4 flex gap-2 items-center">
  <button 
    on:click={resetSelection}
    class="px-3 py-1 bg-gray-700 text-gray-200 rounded text-sm hover:bg-gray-600 transition-colors"
  >
    Reset Selection
  </button>
  <button 
    on:click={refreshTopology}
    class="px-3 py-1 bg-blue-600 text-white rounded text-sm hover:bg-blue-700 transition-colors"
  >
    Refresh
  </button>
  <button 
    on:click={reapplyLayout}
    class="px-3 py-1 bg-purple-600 text-white rounded text-sm hover:bg-purple-700 transition-colors"
  >
    Re-layout
  </button>
  
  <div class="text-xs text-gray-400 ml-auto flex gap-4">
    <span>{graphStats.nodes} nodes</span>
    <span>{graphStats.edges} edges</span>
    <span>Layout: ELK Layered</span>
    <span>Updated: {graphStats.generated_at}</span>
  </div>
</div>

<!-- Svelte Flow Component - Full height -->
<div class="w-full h-[calc(100vh-200px)] bg-gray-900 rounded-lg border border-gray-700 overflow-hidden">
  <SvelteFlow 
    nodes={$nodes} 
    edges={$edges}
    {nodeTypes}
    onnodeclick={onNodeClick}
    onedgeclick={onEdgeClick}
    fitView
    nodesDraggable={true}
    nodesConnectable={false}
    elementsSelectable={true}
    class="bg-gray-900"
  >
    <Background 
      variant={BackgroundVariant.Dots}
      bgColor={"#374151"}
      gap={20} 
      size={1}
    />
    
    <Controls 
      showZoom={true}
      showFitView={true}
      class="bg-gray-800 border border-gray-600 rounded shadow-lg"
    />
    
    <MiniMap 
      nodeColor="#4B5563"
      maskColor="rgba(31, 41, 55, 0.8)"
      class="bg-gray-800 border border-gray-600"
    />
  </SvelteFlow>
</div>

<!-- Node Information Panel -->
{#if selectedNode && selectedNode.data}
  <div class="mt-4 p-4 bg-gray-800 rounded-lg border border-gray-700">
    <div class="flex items-center justify-between mb-3">
      <div class="flex items-center gap-2">
        {#if selectedNode.data.IconComponent}
          <svelte:component 
            this={selectedNode.data.IconComponent as any} 
            class="w-5 h-5 text-gray-300"
          />
        {/if}
        <h3 class="text-sm font-medium text-gray-200">
          {selectedNode.data.label}
        </h3>
        <span class="text-xs px-2 py-1 bg-gray-700 text-gray-300 rounded">
          {selectedNode.type}
        </span>
      </div>
      <button 
        on:click={resetSelection}
        class="text-gray-400 hover:text-gray-200 transition-colors"
      >
        ✕
      </button>
    </div>
    
    <div class="grid grid-cols-2 gap-4 text-xs">
      <div>
        <span class="text-gray-400">ID:</span>
        <span class="text-gray-200 ml-2">{selectedNode.id}</span>
      </div>
      <div>
        <span class="text-gray-400">Type:</span>
        <span class="text-gray-200 ml-2">{selectedNode.data.nodeType}</span>
      </div>
      <div>
        <span class="text-gray-400">Position:</span>
        <span class="text-gray-200 ml-2">
          ({Math.round(selectedNode.position.x)}, {Math.round(selectedNode.position.y)})
        </span>
      </div>
      {#if selectedNode.data.parentId}
        <div>
          <span class="text-gray-400">Parent:</span>
          <span class="text-gray-200 ml-2">{selectedNode.data.parentId}</span>
        </div>
      {/if}
      {#if selectedNode.data.ip}
        <div>
          <span class="text-gray-400">IP:</span>
          <span class="text-gray-200 ml-2">{selectedNode.data.ip}</span>
        </div>
      {/if}
      {#if selectedNode.data.cidr}
        <div>
          <span class="text-gray-400">CIDR:</span>
          <span class="text-gray-200 ml-2">{selectedNode.data.cidr}</span>
        </div>
      {/if}
    </div>

    <!-- Connected Edges -->
    {#if $edges.filter(e => e.source === selectedNode.id || e.target === selectedNode.id).length > 0}
      <div class="mt-3 pt-3 border-t border-gray-700">
        <h4 class="text-xs font-medium text-gray-300 mb-2">Connections</h4>
        <div class="space-y-1">
          {#each $edges.filter(e => e.source === selectedNode.id || e.target === selectedNode.id) as edge}
            <div class="text-xs text-gray-400">
              <span class="text-gray-300">{edge.label || edge.data?.edgeType}</span>: 
              {edge.source === selectedNode.id ? `→ ${edge.target}` : `← ${edge.source}`}
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
{/if}

<style>
  :global(.svelte-flow .svelte-flow__attribution) {
    background: rgba(31, 41, 55, 0.8) !important;
    color: #9CA3AF !important;
    border-radius: 4px !important;
    font-size: 10px !important;
  }
  
  :global(.svelte-flow .svelte-flow__controls button) {
    background: #374151 !important;
    border: 1px solid #4B5563 !important;
    color: #F3F4F6 !important;
  }
  
  :global(.svelte-flow .svelte-flow__controls button:hover) {
    background: #4B5563 !important;
  }
  
  :global(.svelte-flow .svelte-flow__minimap) {
    background: #1F2937 !important;
    border: 1px solid #374151 !important;
  }
</style>