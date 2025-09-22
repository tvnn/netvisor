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
  import { type Node, type Edge, Position, type Position as PositionType } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
  import { getDistanceToNode, getNextHandle, topology } from '../store';
  import { edgeTypes, entities } from '$lib/shared/stores/metadata';
  import { createIconComponent } from '$lib/shared/utils/styling';
  import { pushError } from '$lib/shared/stores/feedback';
  
  // Import custom node components
  import SubnetNode from './SubnetNode.svelte';
  import HostNode from './HostNode.svelte';
	import { EdgeHandle, type TopologyEdgeData } from '../types/base';

  // Define custom edge data type
  interface CustomEdgeData extends Record<string, unknown> {
    edgeType: string;
    label: string;
  }

  // Define node types
  const nodeTypes = {
    SubnetNode: SubnetNode,
    HostNode: HostNode,
  };

  // Stores
  let nodes = writable<Node[]>([]);
  let edges = writable<Edge[]>([]);
  let selectedNodeId: string | null = null;

  // Add debugging to see what's happening
  $: if ($topology?.nodes && $topology?.edges) {
      loadTopologyData();
  }

  function loadTopologyData() {    
    try {
      if ($topology?.nodes && $topology?.edges) {
        
        const flowNodes: Node[] = $topology.nodes.map((node): Node => {
          return {
            id: node.id,
            type: node.node_type,
            position: { x: node.position.x, y: node.position.y },
            width: node.size.x,
            height: node.size.y,
            expandParent: true,
            parentId: node.parent_id || undefined,
            extent: (node.parent_id ? "parent" : undefined),
            data: {
              id: node.id,
              host_id: node.host_id,
              interface_id: node.interface_id,
              infra_width: node.infra_width,
              nodeType: node.node_type,
              parentId: node.parent_id,
              width: node.size.x,
              height: node.size.y,
              subnet_label: node.subnet_label
            },
          };
        });

        const flowEdges: Edge[] = $topology.edges.map(([sourceIdx, targetIdx, edgeData]: [number, number, TopologyEdgeData], index: number): Edge => {
          const edgeType = edgeData.edge_type as string;
          const edgeLabel = edgeTypes.getDisplay(edgeType);
          let edgeColorHelper = edgeTypes.getColorHelper(edgeType);

          const customData: CustomEdgeData = {
            edgeType: edgeType,
            label: edgeLabel,
            sourceHandle: edgeData.source_handle,
            targetHandle: edgeData.target_handle
          };

          return {
            id: `edge-${index}`,
            source: edgeData.source,
            target: edgeData.target,
            sourceHandle: edgeData.source_handle.toString(),
            targetHandle: edgeData.target_handle.toString(),
            type: 'default',
            style: `stroke: ${edgeColorHelper.rgb}; stroke-width: 2px;`,
            data: customData
          };
        });

        setTimeout(() => {
          nodes.set(flowNodes);
          edges.set(flowEdges);
        }, 50);
      }
    } catch (err) {
      pushError(`Failed to parse topology data ${err}`);
    }
  }

  // Event handlers
  function onNodeClick({ node, event }: { node: Node; event: MouseEvent | TouchEvent }) {
    selectedNodeId = node.id;
    console.log('Node clicked:', node);
  }

  function onEdgeClick({ edge, event }: { edge: Edge; event: MouseEvent }) {    
    // Get click coordinates relative to the flow canvas
    const clickX = event.clientX;
    const clickY = event.clientY;
    
    // Find source and target nodes
    const sourceNode = $nodes.find(n => n.id === edge.source);
    const targetNode = $nodes.find(n => n.id === edge.target);
    
    if (!sourceNode || !targetNode) {
      console.warn('Could not find source or target node for edge');
      return;
    }

    // Calculate which node the click was closer to
    const distanceToSource = getDistanceToNode(clickX, clickY, sourceNode);
    const distanceToTarget = getDistanceToNode(clickX, clickY, targetNode);
    const isCloserToSource = distanceToSource < distanceToTarget;    

    // Get current handles from edge data
    const currentTargetHandle = edge.data?.targetHandle as EdgeHandle || EdgeHandle.Top;
    const currentSourceHandle = edge.data?.sourceHandle as EdgeHandle || EdgeHandle.Top;
    
    // Cycle the appropriate handle
    let newSourceHandle = currentSourceHandle;
    let newTargetHandle = currentTargetHandle;
    
    if (isCloserToSource) {
      newSourceHandle = getNextHandle(currentSourceHandle);
    } else {
      newTargetHandle = getNextHandle(currentTargetHandle);
    }
    
    // Update the edge in the edges store
    edges.set($edges.map(e => {
      if (e.id === edge.id) {
        return {
          ...e,
          sourceHandle: newSourceHandle.toString(),
          targetHandle: newTargetHandle.toString(),
          data: {
            ...e.data,
            sourceHandle: newSourceHandle,
            targetHandle: newTargetHandle
          }
        };
      }
      return e;
    }));
  }
</script>

<div class="w-full h-[calc(100vh-200px)] bg-gray-900 rounded-lg border border-gray-700 overflow-hidden">
  <SvelteFlow 
    nodes={$nodes} 
    edges={$edges}
    {nodeTypes}
    onnodeclick={onNodeClick}
    onedgeclick={onEdgeClick}
    fitView
    snapGrid={[25,25]}
    nodesDraggable={true}
    nodesConnectable={false}
    elementsSelectable={true}
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
  </SvelteFlow>
</div>