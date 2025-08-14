<!-- src/lib/components/nodes/NodeCard.svelte -->
<script lang="ts">
  import { Edit, Settings, Trash2, Server, SquareActivity, CircleAlert, TriangleAlert, OctagonAlert } from 'lucide-svelte';
  import type { Node, AssignedTest } from "$lib/types/nodes";
  import { getNodeStatusDisplayName, getNodeStatusColor, getNodeTypeDisplayName } from "$lib/types/nodes";
  import { getTestTypeDisplayName } from "$lib/types/tests";
  import GenericCard from '../common/Card.svelte';
  import type { CardListItem } from '$lib/types';
  
  export let node: Node;
  export let groupNames: string[] = [];
  export let onEdit: (node: Node) => void = () => {};
  export let onDelete: (node: Node) => void = () => {};
  export let onAssignTest: (node: Node) => void = () => {};
  export let onEditTest: (node: Node, test: AssignedTest) => void = () => {};
  // export let onDeleteTest: (node: Node, test: AssignedTest) => void = () => {}; // Removed as it's not used in this implementation
  
  // Get the display status - monitoring status takes precedence if disabled
  function getDisplayStatus() {
    if (!node.monitoring_enabled) {
      return 'Monitoring Disabled';
    }
    return getNodeStatusDisplayName(node.current_status);
  }
  
  // Get the status color - gray for monitoring disabled, otherwise node status color
  function getDisplayStatusColor() {
    if (!node.monitoring_enabled) {
      return 'text-gray-400';
    }
    return getNodeStatusColor(node.current_status);
  }
  
  // Build connection info
  $: connectionInfo = (() => {
    if (node.ip) {
      return `IP: ${node.ip}${node.port ? `:${node.port}` : ''}`;
    } else if (node.domain) {
      return `Domain: ${node.domain}${node.port ? `:${node.port}` : ''}`;
    }
    return '';
  })();
  
  // Build card data
  $: cardData = {
    title: node.name,
    subtitle: node.node_type ? getNodeTypeDisplayName(node.node_type) : 'Unknown Device',
    status: getDisplayStatus(),
    statusColor: getDisplayStatusColor(),
    icon: Server,
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
          label: cap,
          bgColor: 'bg-blue-900/30',
          color: 'text-blue-300'
        })),
        emptyText: 'No capabilities assigned'
      },
      {
        label: 'Groups',
        items: groupNames.map((name, i) => ({
          id: node.node_groups[i] || name,
          label: name,
          bgColor: 'bg-green-900/30',
          color: 'text-green-300'
        })),
        emptyText: 'No groups assigned'
      },
      {
        label: 'Tests',
        items: node.assigned_tests.map(test => ({
          id: test.test_type,
          label: getTestTypeDisplayName(test.test_type),
          metadata: test,
          disabled: !test.enabled
        })),
        emptyText: 'No tests assigned',
        renderItem: (item: CardListItem) => {
          const test = item.metadata as AssignedTest;
          const icon = test.criticality === 'Critical' ? 'text-red-400' :
                      test.criticality === 'Important' ? 'text-yellow-300' : 'text-blue-300';
          const iconComponent = test.criticality === 'Critical' ? 'OctagonAlert' :
                               test.criticality === 'Important' ? 'TriangleAlert' : 'CircleAlert';
          
          return `
            <div class="flex items-center space-x-2">
              <svg class="w-4 h-4 ${icon}" fill="currentColor" viewBox="0 0 24 24">
                ${test.criticality === 'Critical' ? 
                  '<path d="M12 2L2 22h20L12 2zm0 4l7.5 13h-15L12 6z"/>' :
                  test.criticality === 'Important' ?
                  '<path d="M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z"/>' :
                  '<circle cx="12" cy="12" r="10"/>'
                }
              </svg>
              <span class="text-gray-300">${item.label}</span>
              ${item.disabled ? '<span class="text-xs text-gray-500">(disabled)</span>' : ''}
              ${test.monitor_interval_minutes ? `<span class="text-xs text-gray-500">${test.monitor_interval_minutes}m</span>` : ''}
            </div>
          `;
        },
        itemActions: (item: CardListItem) => [{
          label: 'Edit Test',
          icon: Settings,
          color: 'text-gray-400',
          hoverColor: 'text-white',
          bgHover: 'hover:bg-gray-700',
          onClick: () => onEditTest(node, item.metadata as AssignedTest)
        }]
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
        label: 'Assign Test',
        icon: SquareActivity,
        color: 'text-gray-400',
        hoverColor: 'text-white',
        bgHover: 'hover:bg-gray-700',
        onClick: () => onAssignTest(node)
      },
      {
        label: 'Edit Node',
        icon: Settings,
        color: 'text-gray-400',
        hoverColor: 'text-white',
        bgHover: 'hover:bg-gray-700',
        onClick: () => onEdit(node)
      }
    ]
  };
</script>

<GenericCard {...cardData} />