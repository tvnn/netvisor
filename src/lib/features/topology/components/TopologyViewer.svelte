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
  import { edgeTypes } from '$lib/shared/stores/registry';
  import { createIconComponent } from '$lib/shared/utils/styling';
  import { pushError } from '$lib/shared/stores/feedback';
  
  // Import custom node components
  import SubnetNode from './SubnetNode.svelte';
  import HostNode from './HostNode.svelte';
	import InterfaceNode from './InterfaceNode.svelte';

  // Define custom edge data type
  interface CustomEdgeData extends Record<string, unknown> {
    edgeType: string;
    label: string;
  }

  // Define node types
  const nodeTypes = {
    SubnetNode: SubnetNode,
    HostNode: HostNode,
    InterfaceNode: InterfaceNode
  };

  // Stores
  let nodes = writable<Node[]>([]);
  let edges = writable<Edge[]>([]);
  let selectedNodeId: string | null = null;

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
        const flowNodes: Node[] = $topology.nodes.map((node: any): Node => {
          return {
            id: node.id,
            type: node.node_type,
            position: { x: node.position.x, y: node.position.y },
            parentId: node.parent_id || undefined,
            extent: (node.parent_id ? "parent" : undefined),
            width: node.size.x,
            height: node.size.y,
            data: {
              label: node.label,
              id: node.id,
              nodeType: node.node_type,
              color: node.color,
              icon: node.icon,
              parentId: node.parent_id,
              width: node.size.x,
              height: node.size.y,
              IconComponent: createIconComponent(node.icon),
            },
          };
        });

        const flowEdges: Edge[] = $topology.edges.map(([sourceIdx, targetIdx, edgeData]: [number, number, any], index: number): Edge => {
          const edgeType = edgeData.edge_type as string;
          const edgeColor = edgeTypes.getColorHelper(edgeType).bg;
          const edgeLabel = edgeTypes.getDisplay(edgeType);
          
          const customData: CustomEdgeData = {
            edgeType: edgeType,
            label: edgeLabel
          };

          return {
            id: `edge-${index}`,
            source: edgeData.source,
            target: edgeData.target,
            type: 'smoothstep',
            style: `stroke-width: 2px;`,
            markerEnd: {
              type: MarkerType.ArrowClosed,
              color: edgeColor
            },
            data: customData
          };
        });

        nodes.set(flowNodes);
        edges.set(flowEdges);
      }
    } catch (err) {
      pushError('Failed to parse topology data');
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
</script>

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
      gap={10} 
      size={1}
    />
    
    <Controls 
      showZoom={true}
      showFitView={true}
      class="!bg-gray-800 !border !border-gray-600 !rounded !shadow-lg [&_button]:!bg-gray-700 [&_button]:!border-gray-600 [&_button]:!text-gray-100 [&_button:hover]:!bg-gray-600"
    />
    
    <MiniMap 
      nodeColor="#4B5563"
      maskColor="rgba(31, 41, 55, 0.8)"
      class="!bg-gray-800 !border !border-gray-600"
    />
  </SvelteFlow>
</div>