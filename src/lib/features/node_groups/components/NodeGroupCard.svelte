<script lang="ts">
  import { Edit, Trash2, Play } from 'lucide-svelte';
	import type { NodeGroup } from '../types/base';
    import type { Node } from "$lib/features/nodes/types/base";
	import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
  
  export let group: NodeGroup;
  export let nodes: Node[] = [];
  export let onEdit: (group: NodeGroup) => void = () => {};
  export let onDelete: (group: NodeGroup) => void = () => {};
  export let onExecute: (group: NodeGroup) => void = () => {};
  
  // Get node name from ID
  function getNodeName(nodeId: string): string {
    const node = nodes.find(n => n.id === nodeId);
    return node ? node.name : `Node ${nodeId.slice(0, 8)}...`;
  }
  
  // Get status info
  function getStatusInfo() {
    if (group.auto_diagnostic_enabled) {
      return {
        status: 'Auto-Diagnostic Enabled',
        color: 'text-green-400'
      };
    } else {
      return {
        status: 'Manual Only',
        color: 'text-gray-400'
      };
    }
  }
  
  $: statusInfo = getStatusInfo();
  
  // Build card data
  $: cardData = {
    title: group.name,
    subtitle: `${group.node_sequence.length} nodes in sequence`,
    status: statusInfo.status,
    statusColor: statusInfo.color,
    
    sections: group.description ? [{
      label: 'Description',
      value: group.description
    }] : [],
    
    lists: [
      {
        label: 'Diagnostic Sequence',
        items: group.node_sequence.map((nodeId, index) => ({
          id: nodeId,
          label: `${index + 1}. ${getNodeName(nodeId)}`,
          bgColor: 'bg-purple-900/30',
          color: 'text-purple-300'
        })),
        emptyText: 'No nodes in sequence'
      }
    ],
    
    actions: [
      {
        label: 'Delete Group',
        icon: Trash2,
        color: 'text-gray-400',
        hoverColor: 'text-red-400',
        bgHover: 'hover:bg-red-900/20',
        onClick: () => onDelete(group)
      },
      // {
      //   label: 'Execute Diagnostic',
      //   icon: Play,
      //   color: 'text-gray-400',
      //   hoverColor: 'text-green-400',
      //   bgHover: 'hover:bg-green-900/20',
      //   onClick: () => onExecute(group),
      //   disabled: group.node_sequence.length === 0
      // },
      {
        label: 'Edit Group',
        icon: Edit,
        color: 'text-gray-400',
        hoverColor: 'text-white',
        bgHover: 'hover:bg-gray-700',
        onClick: () => onEdit(group)
      }
    ]
  };
</script>

<GenericCard {...cardData} />