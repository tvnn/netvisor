<script lang="ts">
  import { CircleQuestionMark, Edit, Radar, Replace, Trash2 } from 'lucide-svelte';
	import { getHostTargetString } from '../store';
  import type { Host } from '../types/base';
	import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
	import type { Daemon } from '$lib/features/daemons/types/base';
	import { getDaemonDiscoveryState } from '$lib/features/daemons/store';
  import DaemonDiscoveryStatus from '$lib/features/discovery/DaemonDiscoveryStatus.svelte';
	import { sessions } from '$lib/features/discovery/store';
	import { entities, serviceDefinitions } from '$lib/shared/stores/metadata';
	import { subnets } from '$lib/features/subnets/store';
	import { get } from 'svelte/store';
	import type { Group } from '$lib/features/groups/types/base';
	import { getServicesForHost, services } from '$lib/features/services/store';

  export let host: Host;
  export let daemon: Daemon | null;
  export let hostGroups: Group[] = [];
  export let onEdit: (host: Host) => void = () => {};
  export let onDelete: (host: Host) => void = () => {};
  export let onDiscovery: (daemon: Daemon) => void = () => {};
  export let onConsolidate: (host: Host) => void = () => {};
  export let discoveryIsRunning: boolean;

  // Build connection info
  $: connectionInfo = getHostTargetString(host)

  $: hostIsRunningDiscovery = (discoveryIsRunning && daemon !== null) ? getDaemonDiscoveryState(daemon.id, $sessions) !== null : false;
  $: discoveryData = hostIsRunningDiscovery && daemon ? getDaemonDiscoveryState(daemon.id, $sessions) : null;

  $: hostServices = (() => {
    // Force reactivity to services store
    $services; 
    return getServicesForHost(host.id);
  })();

  function getSubnetNameFromId(id: string): string | null {
    return get(subnets).find(s => s.id == id)?.cidr || null
  }
  
  // Build card data
  $: cardData = {
    title: host.name,
    iconColor: entities.getColorHelper("Host").icon,
    icon: serviceDefinitions.getIconComponent(hostServices[0]?.service_definition) || entities.getIconComponent("Host"),
    sections: connectionInfo ? [{
      label: 'Connection',
      value: connectionInfo,
      link: `http://${connectionInfo}`
    }] : [],
    
    lists: [
      {
        label: 'Services',
        items: hostServices.map(sv => {
          return ({
            id: sv.service_definition,
            label: serviceDefinitions.getName(sv.service_definition),
            color: entities.getColorHelper("Service").string
          })
        }),
        emptyText: 'No services assigned'
      },
      {
        label: 'Subnets',
        items: host.interfaces.map(sub => {
          return ({
            id: sub.subnet_id,
            label: getSubnetNameFromId(sub.subnet_id) || "Unknown Subnet",
            color: entities.getColorHelper("Subnet").string
          })
        }),
        emptyText: 'No subnets assigned'
      },
      {
        label: 'Groups',
        items: hostGroups.map((group: Group, i) => ({
          id: group.id,
          label: group.name,
          color: entities.getColorHelper("Group").string
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
            label: 'Consolidate',
            icon: Replace,
            color: 'text-gray-400',
            hoverColor: 'text-white',
            bgHover: 'hover:bg-gray-700',
            onClick: () => onConsolidate(host),
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