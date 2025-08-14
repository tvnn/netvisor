<script lang="ts">
  import { Edit, Trash2, Server, CircleAlert, TriangleAlert, OctagonAlert } from 'lucide-svelte';
  import type { Node } from "$lib/types/nodes";
  import { getNodeStatusDisplayName, getNodeStatusColor } from "$lib/config/nodes/status";
  import { getNodeTypeDisplay, getNodeTypeIcon } from "$lib/config/nodes/types";
  import { getTestDisplay } from "$lib/config/tests/types";
  import GenericCard from '../common/Card.svelte';
  
  export let node: Node;
  export let groupNames: string[] = [];
  export let onEdit: (node: Node) => void = () => {};
  export let onDelete: (node: Node) => void = () => {};
  
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
  
  // Get criticality icon and color
  function getCriticalityIconConfig(criticality: string) {
    switch (criticality) {
      case 'Critical':
        return { icon: OctagonAlert, color: 'text-red-400' };
      case 'Important':
        return { icon: TriangleAlert, color: 'text-yellow-300' };
      case 'Informational':
        return { icon: CircleAlert, color: 'text-blue-300' };
      default:
        return { icon: CircleAlert, color: 'text-gray-400' };
    }
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
    subtitle: getNodeTypeDisplay(node.node_type),
    status: getDisplayStatus(),
    statusColor: getDisplayStatusColor(),
    icon: getNodeTypeIcon(node.node_type),
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
        label: 'Diagnostic Groups',
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
        items: node.assigned_tests.map(test => {
          const criticalityConfig = getCriticalityIconConfig(test.criticality);
          return {
            id: test.test_type,
            label: getTestDisplay(test.test_type),
            icon: criticalityConfig.icon,
            iconColor: criticalityConfig.color,
            bgColor: test.enabled ? 'bg-gray-700/50' : 'bg-gray-700/30',
            color: test.enabled ? 'text-gray-300' : 'text-gray-500',
            disabled: !test.enabled,
            badge: test.monitor_interval_minutes ? `${test.monitor_interval_minutes}m` : undefined,
            badgeColor: 'text-gray-500',
            metadata: test
          };
        }),
        emptyText: 'No tests assigned'
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