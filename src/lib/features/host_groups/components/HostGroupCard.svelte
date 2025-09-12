<script lang="ts">
  import { Edit, Trash2, Users } from 'lucide-svelte';
  import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
  import { hosts } from '$lib/features/hosts/store';
  import { get } from 'svelte/store';
  import type { HostGroup } from '../types/base';
  
  export let group: HostGroup;
  export let onEdit: (group: HostGroup) => void = () => {};
  export let onDelete: (group: HostGroup) => void = () => {};
    
  function getHostName(id: string): string | null {
    return get(hosts).find(h => h.id === id)?.name || null;
  }
  
  // Build card data
  $: cardData = {
    title: group.name,
    subtitle: `${group.hosts.length} host${group.hosts.length === 1 ? '' : 's'} in group`,
    iconColor: 'text-purple-400',
    icon: Users,
    
    sections: group.description ? [{
      label: 'Description',
      value: group.description
    }] : [],
    
    lists: [
      {
        label: 'Hosts',
        items: group.hosts.map((hostId) => ({
          id: hostId,
          label: getHostName(hostId) || "Unknown Host",
          color: 'blue'
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