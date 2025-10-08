<script lang="ts">
	import { writable } from 'svelte/store';
	import { SvelteFlow, Controls, Background, BackgroundVariant } from '@xyflow/svelte';
	import { type Node, type Edge } from '@xyflow/svelte';
	import '@xyflow/svelte/dist/style.css';
	import { getDistanceToNode, getNextHandle, topology } from '../store';
	import { edgeTypes, entities } from '$lib/shared/stores/metadata';
	import { pushError } from '$lib/shared/stores/feedback';

	// Import custom node components
	import SubnetNode from './SubnetNode.svelte';
	import HostNode from './HostNode.svelte';
	import {
		EdgeHandle,
		type TopologyEdgeData,
		type CustomEdgeData,
		type CustomNodeData
	} from '../types/base';
	import { twColorToRgba } from '$lib/shared/utils/styling';

	// Define node types
	const nodeTypes = {
		SubnetNode: SubnetNode,
		HostNode: HostNode
	};

	// Stores
	let nodes = writable<Node[]>([]);
	let edges = writable<Edge[]>([]);
	// let selectedNodeId: string | null = null;

	// Add debugging to see what's happening
	$: if ($topology?.nodes && $topology?.edges) {
		loadTopologyData();
	}

	function loadTopologyData() {
		try {
			if ($topology?.nodes && $topology?.edges) {
				const flowNodes: Node[] = $topology.nodes.map((node): Node => {
					const data: CustomNodeData = {
						id: node.id,
						host_id: node.host_id,
						interface_id: node.interface_id,
						infra_width: node.infra_width,
						nodeType: node.node_type,
						parentId: node.subnet_id,
						width: node.size.x,
						height: node.size.y,
						subnet_type: node.subnet_type
					};

					return {
						id: node.id,
						type: node.node_type,
						position: { x: node.position.x, y: node.position.y },
						width: node.size.x,
						height: node.size.y,
						expandParent: true,
						deletable: false,
						parentId: node.subnet_id || undefined,
						extent: node.subnet_id ? 'parent' : undefined,
						data
					};
				});

				const flowEdges: Edge[] = $topology.edges.map(
					([, , edgeData]: [number, number, TopologyEdgeData], index: number): Edge => {
						const edgeType = edgeData.edge_type as string;
						const edgeLabel = edgeTypes.getName(edgeType);
						let edgeMetadata = edgeTypes.getMetadata(edgeType);
						let edgeColorHelper = edgeTypes.getColorHelper(edgeType);
						let hostColorHelper = entities.getColorHelper('Host');

						const dashArray = edgeMetadata.is_dashed ? 'stroke-dasharray: 5,5;' : '';
						const labelStyle =
							edgeType === 'Interface'
								? `background: ${twColorToRgba(hostColorHelper.bg)};
								color: ${hostColorHelper.rgb};
								border: 2px solid ${twColorToRgba(hostColorHelper.border)};`
								: 'background: #374151; color: #f3f4f6; border: 1px solid #4b5563;';

						const data: CustomEdgeData = {
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
							type: 'smoothstep',
							label: edgeData.label,
							labelStyle:
								labelStyle +
								'font-size: 12px; font-weight: 500; padding: 2px 6px; border-radius: 4px;',
							style: `stroke: ${edgeColorHelper.rgb}; stroke-width: 2px; ${dashArray}`,
							data
						};
					}
				);

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
	function onNodeClick({ node }: { node: Node; event: MouseEvent | TouchEvent }) {
		// selectedNodeId = node.id;
		console.log('Node clicked:', node);
	}

	function onEdgeClick({ edge, event }: { edge: Edge; event: MouseEvent }) {
		// Get click coordinates relative to the flow canvas
		const clickX = event.clientX;
		const clickY = event.clientY;

		// Find source and target nodes
		const sourceNode = $nodes.find((n) => n.id === edge.source);
		const targetNode = $nodes.find((n) => n.id === edge.target);

		if (!sourceNode || !targetNode) {
			console.warn('Could not find source or target node for edge');
			return;
		}

		// Calculate which node the click was closer to
		const distanceToSource = getDistanceToNode(clickX, clickY, sourceNode);
		const distanceToTarget = getDistanceToNode(clickX, clickY, targetNode);
		const isCloserToSource = distanceToSource < distanceToTarget;

		// Get current handles from edge data
		const currentTargetHandle = (edge.data?.targetHandle as EdgeHandle) || EdgeHandle.Top;
		const currentSourceHandle = (edge.data?.sourceHandle as EdgeHandle) || EdgeHandle.Top;

		// Cycle the appropriate handle
		let newSourceHandle = currentSourceHandle;
		let newTargetHandle = currentTargetHandle;

		if (isCloserToSource) {
			newSourceHandle = getNextHandle(currentSourceHandle);
		} else {
			newTargetHandle = getNextHandle(currentTargetHandle);
		}

		// Update the edge in the edges store
		edges.set(
			$edges.map((e) => {
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
			})
		);
	}
</script>

<div class="h-[calc(100vh-200px)] w-full overflow-hidden rounded-lg border border-gray-700">
	<SvelteFlow
		nodes={$nodes}
		edges={$edges}
		{nodeTypes}
		onnodeclick={onNodeClick}
		onedgeclick={onEdgeClick}
		fitView
		snapGrid={[25, 25]}
		nodesDraggable={true}
		nodesConnectable={false}
		elementsSelectable={true}
	>
		<Background variant={BackgroundVariant.Dots} bgColor="#374151" gap={10} size={1} />

		<Controls
			showZoom={true}
			showFitView={true}
			class="!rounded !border !border-gray-600 !bg-gray-800 !shadow-lg [&_button:hover]:!bg-gray-600 [&_button]:!border-gray-600 [&_button]:!bg-gray-700 [&_button]:!text-gray-100"
		/>
	</SvelteFlow>
</div>

<style>
	:global(.svelte-flow.hide-for-export .svelte-flow__controls),
	:global(.svelte-flow.hide-for-export .svelte-flow__resize-control),
	:global(.svelte-flow.hide-for-export .svelte-flow__minimap),
	:global(.svelte-flow.hide-for-export .svelte-flow__panel) {
		display: none !important;
	}
</style>
