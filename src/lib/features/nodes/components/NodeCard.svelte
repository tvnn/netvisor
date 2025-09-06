<script lang="ts">
  import { Edit, Radar, Trash2 } from 'lucide-svelte';
	import { getNodeTargetString } from '../store';
  import type { Node } from '../types/base';
	import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
	import type { Daemon } from '$lib/features/daemons/types/base';
	import { getDaemonDiscoveryState } from '$lib/features/daemons/store';
  import DaemonDiscoveryStatus from '$lib/features/discovery/DaemonDiscoveryStatus.svelte';
	import { sessions } from '$lib/features/discovery/store';
	import { services } from '$lib/shared/stores/registry';
  
  export let node: Node;
  export let daemon: Daemon | null;
  export let groupInfo: any[] = [];
  export let onEdit: (node: Node) => void = () => {};
  export let onDelete: (node: Node) => void = () => {};
  export let onDiscovery: (daemon: Daemon) => void = () => {};
  export let discoveryIsRunning: boolean;
  
  // Build connection info
  $: connectionInfo = getNodeTargetString(node.target)

  $: nodeIsRunningDiscovery = (discoveryIsRunning && daemon !== null) ? getDaemonDiscoveryState(daemon.id, $sessions) !== null : false;
  $: discoveryData = nodeIsRunningDiscovery && daemon ? getDaemonDiscoveryState(daemon.id, $sessions) : null;
  
  // Build card data
  $: cardData = {
    title: node.name,
    iconColor: 'text-blue-400',
    icon: services.getIconComponent(node.services[0].type),
    sections: connectionInfo ? [{
      label: 'Connection',
      value: connectionInfo
    }] : [],
    
    lists: [
      {
        label: 'Services',
        items: node.services.map(cap => {
          return ({
            id: cap.type,
            label: services.getDisplay(cap.type),
            bgColor: 'bg-purple-900/30',
            color: 'text-purple-300'
          })
        }),
        emptyText: 'No capabilities assigned'
      },
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
            color: nodeIsRunningDiscovery ? 'text-green-400' : 'text-gray-400',
            hoverColor: nodeIsRunningDiscovery ? 'text-green-400' : (discoveryIsRunning ? 'text-gray-400' : 'text-green-400'),
            bgHover: nodeIsRunningDiscovery ? 'hover:bg-green-700/50': (discoveryIsRunning ? '' : 'hover:bg-green-700/50'),
            onClick: !nodeIsRunningDiscovery ? () => onDiscovery(daemon) : () => {},
            animation: nodeIsRunningDiscovery ? 'animate-spin' : '',
            disabled: nodeIsRunningDiscovery
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
    ],
    
    // Add footer when discovery is running
    footerComponent: nodeIsRunningDiscovery && daemon ? DaemonDiscoveryStatus : null,
    footerProps: nodeIsRunningDiscovery && daemon ? {
      daemon,
      discoveryData
    } : {}
  };
</script>

<GenericCard {...cardData} />