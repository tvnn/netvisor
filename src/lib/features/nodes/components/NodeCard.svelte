<script lang="ts">
  import { Edit, Radar, Trash2 } from 'lucide-svelte';
	import { getNodeTargetString } from '../store';
  import type { Node } from '../types/base';
	import { capabilities, nodeStatuses, nodeTypes } from '$lib/shared/stores/registry';
	import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
	import type { Daemon } from '$lib/features/daemons/types/base';
	import { getDaemonDiscoveryState } from '$lib/features/daemons/store';
  
  export let node: Node;
  export let daemon: Daemon | null;
  export let groupInfo: any[] = [];
  export let onEdit: (node: Node) => void = () => {};
  export let onDelete: (node: Node) => void = () => {};
  export let onDiscovery: (daemon: Daemon) => void = () => {};
  export let discoveryIsRunning: boolean;
  
  // Build connection info
  $: connectionInfo = getNodeTargetString(node.target)

  $: nodeIsRunningDiscovery = (discoveryIsRunning && daemon !== null) ? getDaemonDiscoveryState(daemon.id) !== null : false;
  
  // Build card data
  $: cardData = {
    title: node.name,
    subtitle: nodeTypes.getDisplay(node.node_type),
    status: node.monitoring_interval == 0 ? 'Monitoring Disabled' : nodeStatuses.getDisplay(node.status || null),
    statusColor: node.monitoring_interval == 0 ? 'text-gray-400' : nodeStatuses.getColor(node.status || null).text,
    icon: nodeTypes.getIconComponent(node.node_type),
    iconColor: 'text-blue-400',
    
    sections: connectionInfo ? [{
      label: 'Connection',
      value: connectionInfo
    }] : [],
    
    lists: [
      {
        label: 'Capabilities',
        items: node.capabilities.map(cap => {

          const [capId, capInfo] = Object.entries(cap)[0];

          return ({
            id: capId,
            label: capabilities.getDisplay(capId),
            bgColor: 'bg-purple-900/30',
            color: 'text-purple-300'
          })
        }),
        emptyText: 'No capabilities assigned'
      },
      // {
      //   label: 'Tests',
      //   items: node.assigned_tests.map((assigned,i) => {
      //     return {
      //       id: assigned.test.type,
      //       label: `${i + 1}. ${$getTestDisplay(assigned.test.type)}`,
      //       disabled: (node.monitoring_interval == 0),
      //       bgColor:  getBgColor( $getCriticalityColor(assigned.criticality) ),
      //       color: getTextColor( $getCriticalityColor(assigned.criticality) ),
      //       badgeColor: 'text-gray-500',
      //       metadata: assigned
      //     };
      //   }),
      //   emptyText: 'No tests assigned'
      // },
            {
        label: 'Diagnostic Groups',
        items: groupInfo.map((group, i) => ({
          id: node?.node_groups ? node.node_groups[i] : group.name,
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
      ...(daemon !== null
        ? [{
            label: 'Run Discovery',
            icon: Radar,
            color: nodeIsRunningDiscovery ? 'text-green' : 'text-gray-400',
            hoverColor: nodeIsRunningDiscovery ? 'text-green' : (discoveryIsRunning ? 'text-gray-400' : 'text-green'),
            bgHover: nodeIsRunningDiscovery ? 'hover:bg-green-700/50': (discoveryIsRunning ? '' : 'hover:bg-green-700/50'),
            onClick: nodeIsRunningDiscovery ? () => onDiscovery(daemon) : () => {},
            animation: nodeIsRunningDiscovery ? 'animate-spin' : ''
          }]
        : []
      ),
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