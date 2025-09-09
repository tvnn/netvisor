<script lang="ts">
  import { Edit, Trash2, Play } from 'lucide-svelte';
	import type { HostGroup } from '../types/base';
    import type { Host } from "$lib/features/hosts/types/base";
	import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
  
  export let group: HostGroup;
  export let hosts: Host[] = [];
  export let onEdit: (group: HostGroup) => void = () => {};
  export let onDelete: (group: HostGroup) => void = () => {};
  
  // Get host name from ID
  function getHostName(hostId: string): string {
    const host = hosts.find(n => n.id === hostId);
    return host ? host.name : `Host ${hostId.slice(0, 8)}...`;
  }
      
  // Build card data
  $: cardData = {
    title: group.name,
    subtitle: `${group.hosts.length} hosts in group`,
    
    sections: group.description ? [{
      label: 'Description',
      value: group.description
    }] : [],
    
    lists: [
      {
        label: 'Hosts',
        items: group.hosts.map((hostId, index) => ({
          id: hostId,
          label: `${index + 1}. ${getHostName(hostId)}`,
          bgColor: 'bg-purple-900/30',
          color: 'text-purple-300'
        })),
        emptyText: 'No hosts in group'
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
      //   disabled: group.hosts.length === 0
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