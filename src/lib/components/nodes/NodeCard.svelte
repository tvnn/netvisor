<script lang="ts">
  import { Edit, Trash2 } from 'lucide-svelte';
  import { getNodeTargetString, type Node } from "$lib/components/nodes/types";
  import GenericCard from '../common/Card.svelte';
	import { getCapabilityDisplay, getCriticalityColor, getNodeStatusColor, getNodeStatusDisplay, getNodeTypeDisplay, getNodeTypeIcon, getTestDisplay } from '$lib/api/registry';
	import { getBgColor, getTextColor } from '../common/colors';
  
  export let node: Node;
  export let groupInfo: any[] = [];
  export let onEdit: (node: Node) => void = () => {};
  export let onDelete: (node: Node) => void = () => {};
  
  // Get the display status - monitoring status takes precedence if disabled
  function getDisplayStatus() {
    if (node.monitoring_interval == 0) {
      return 'Monitoring Disabled';
    }
    return $getNodeStatusDisplay(node.current_status);
  }
  
  // Get the status color - gray for monitoring disabled, otherwise node status color
  function getDisplayStatusColor() {
    if (node.monitoring_interval == 0) {
      return 'text-gray-400';
    }
    return $getNodeStatusColor(node.current_status);
  }
  
  // Build connection info
  $: connectionInfo = getNodeTargetString(node.target)
  
  // Build card data
  $: cardData = {
    title: node.name,
    subtitle: $getNodeTypeDisplay(node.node_type),
    status: getDisplayStatus(),
    statusColor: getDisplayStatusColor(),
    icon: $getNodeTypeIcon(node.node_type),
    iconColor: 'text-blue-400',
    
    sections: connectionInfo ? [{
      label: 'Connection',
      value: connectionInfo
    }] : [],
    
    lists: [
      {
        label: 'Capabilities',
        items: node.capabilities.map(cap => ({
          id: cap,
          label: $getCapabilityDisplay(cap),
          bgColor: 'bg-purple-900/30',
          color: 'text-purple-300'
        })),
        emptyText: 'No capabilities assigned'
      },
      {
        label: 'Tests',
        items: node.assigned_tests.map((assigned,i) => {
          return {
            id: assigned.test.type,
            label: `${i + 1}. ${$getTestDisplay(assigned.test.type)}`,
            disabled: (node.monitoring_interval == 0),
            bgColor:  getBgColor( $getCriticalityColor(assigned.criticality) ),
            color: getTextColor( $getCriticalityColor(assigned.criticality) ),
            badgeColor: 'text-gray-500',
            metadata: assigned
          };
        }),
        emptyText: 'No tests assigned'
      },
            {
        label: 'Diagnostic Groups',
        items: groupInfo.map((group, i) => ({
          id: node.node_groups[i] || group.name,
          label: group.name,
          disabled: !group.auto_diagnostic_enabled,
          bgColor: 'bg-green-900/30',
          color: 'text-green-400'
        })),
        emptyText: 'No groups assigned'
      }
    ],
    
    actions: [
      {
        label: 'Delete Node',
        icon: Trash2,
        color: 'text-gray-400',
        hoverColor: 'text-red-400',
        bgHover: 'hover:bg-red-900/20',
        onClick: () => onDelete(node)
      },
      {
        label: 'Edit Node',
        icon: Edit,
        color: 'text-gray-400',
        hoverColor: 'text-white',
        bgHover: 'hover:bg-gray-700',
        onClick: () => onEdit(node)
      }
    ]
  };
</script>

<GenericCard {...cardData} />