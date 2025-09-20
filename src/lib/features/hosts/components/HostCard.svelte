<script lang="ts">
  import { CircleQuestionMark, Edit, Radar, Replace, Trash2 } from 'lucide-svelte';
	import { getHostTargetString } from '../store';
  import type { Host } from '../types/base';
	import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
	import type { Daemon } from '$lib/features/daemons/types/base';
	import { getDaemonDiscoveryState } from '$lib/features/daemons/store';
  import DaemonDiscoveryStatus from '$lib/features/discovery/DaemonDiscoveryStatus.svelte';
	import { sessions } from '$lib/features/discovery/store';
	import { serviceTypes } from '$lib/shared/stores/registry';
	import { subnets } from '$lib/features/subnets/store';
	import { get } from 'svelte/store';
	import type { HostGroup } from '$lib/features/host_groups/types/base';
	import { getServicesForHost } from '$lib/features/services/store';
  
  export let host: Host;
  export let daemon: Daemon | null;
  export let groupInfo: any[] = [];
  export let onEdit: (host: Host) => void = () => {};
  export let onDelete: (host: Host) => void = () => {};
  export let onDiscovery: (daemon: Daemon) => void = () => {};
  export let onConvert: (host: Host) => void = () => {};
  export let discoveryIsRunning: boolean;
  
  // Build connection info
  $: connectionInfo = getHostTargetString(host)

  $: hostIsRunningDiscovery = (discoveryIsRunning && daemon !== null) ? getDaemonDiscoveryState(daemon.id, $sessions) !== null : false;
  $: discoveryData = hostIsRunningDiscovery && daemon ? getDaemonDiscoveryState(daemon.id, $sessions) : null;

  $: hostServices = get(getServicesForHost(host.id));

  function getSubnetNameFromId(id: string): string | null {
    return get(subnets).find(s => s.id == id)?.cidr || null
  }
  
  // Build card data
  $: cardData = {
    title: host.name,
    iconColor: 'text-blue-400',
    icon: serviceTypes.getIconComponent(hostServices[0]?.service_type.type) || CircleQuestionMark,
    sections: connectionInfo ? [{
      label: 'Connection',
      value: connectionInfo
    }] : [],
    
    lists: [
      {
        label: 'Services',
        items: hostServices.map(sv => {
          return ({
            id: sv.service_type.type,
            label: serviceTypes.getDisplay(sv.service_type.type),
            color: 'cyan'
          })
        }),
        emptyText: 'No services assigned'
      },
      {
        label: 'Subnets',
        items: host.interfaces.filter(sub => getSubnetNameFromId(sub.subnet_id) != null).map(sub => {
          return ({
            id: sub.subnet_id,
            label: getSubnetNameFromId(sub.subnet_id) || "Unknown Subnet",
            color: 'orange'
          })
        }),
        emptyText: 'No subnets assigned'
      },
      {
        label: 'Groups',
        items: groupInfo.map((group: HostGroup, i) => ({
          id: host?.groups ? host.groups[i] : group.name,
          label: group.name,
          color: 'purple'
        })),
        emptyText: 'No groups assigned'
      }
    ],
    
    actions: [
      {
        label: 'Delete Host',
        icon: Trash2,
        color: 'text-gray-400',
        hoverColor: 'text-red-400',
        bgHover: 'hover:bg-red-900/20',
        onClick: () => onDelete(host)
      },
      ...(daemon == null
        ? [{
            label: 'Convert to Interface',
            icon: Replace,
            color: 'text-gray-400',
            hoverColor: 'text-white',
            bgHover: 'hover:bg-gray-700',
            onClick: () => onConvert(host),
          }]
        : []
      ),
      ...(daemon !== null
        ? [{
            label: 'Run Discovery',
            icon: Radar,
            color: hostIsRunningDiscovery ? 'text-green-400' : 'text-gray-400',
            hoverColor: hostIsRunningDiscovery ? 'text-green-400' : (discoveryIsRunning ? 'text-gray-400' : 'text-green-400'),
            bgHover: hostIsRunningDiscovery ? 'hover:bg-green-700/50': (discoveryIsRunning ? '' : 'hover:bg-green-700/50'),
            onClick: !hostIsRunningDiscovery ? () => onDiscovery(daemon) : () => {},
            animation: hostIsRunningDiscovery ? 'animate-spin' : '',
            disabled: hostIsRunningDiscovery
          }]
        : []
      ),
      {
        label: 'Edit Host',
        icon: Edit,
        color: 'text-gray-400',
        hoverColor: 'text-white',
        bgHover: 'hover:bg-gray-700',
        onClick: () => onEdit(host)
      }
    ],
    
    // Add footer when discovery is running
    footerComponent: hostIsRunningDiscovery && daemon ? DaemonDiscoveryStatus : null,
    footerProps: hostIsRunningDiscovery && daemon ? {
      daemon,
      discoveryData
    } : {}
  };
</script>

<GenericCard {...cardData} />