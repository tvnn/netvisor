<script lang="ts">
  import { Edit, Trash2, Network } from 'lucide-svelte';
	import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
	import { nodes } from '$lib/features/nodes/store';
	import { get } from 'svelte/store';
	import { subnet_types } from '$lib/shared/stores/registry';
  
  export let subnet: Subnet;
  export let onEdit: (subnet: Subnet) => void = () => {};
  export let onDelete: (subnet: Subnet) => void = () => {};
    
  function getNodeName(id: string): string | null {
    return get(nodes).find(n => n.id == id)?.name || null
  }
  
  // Build card data
  $: cardData = {
    title: subnet.name,
    subtitle: subnet.cidr,
    iconColor: 'text-orange-400',
    icon: Network,
    
    sections: subnet.description ? [{
      label: 'Description',
      value: subnet.description
    }] : [],
    
    lists: [
      {
        label: 'Network Type',
        items: [{
          id: 'type',
          label: subnet_types.getDisplay(subnet.subnet_type),
          color: subnet_types.getColorString(subnet.subnet_type)
        }],
        emptyText: 'No type specified'
      },
      ...(subnet.dns_resolvers && subnet.dns_resolvers.length > 0 ? [{
        label: 'DNS Resolvers',
        items: subnet.dns_resolvers.map((resolverId) => ({
          id: resolverId,
          label: getNodeName(resolverId) || "Unknown Node",
          color: 'yellow'
        })),
        emptyText: 'No DNS resolvers'
      }] : []),
      ...(subnet.gateways && subnet.gateways.length > 0 ? [{
        label: 'Gateways',
        items: subnet.gateways.map((gatewayId) => ({
          id: gatewayId,
          label: getNodeName(gatewayId) || "Unknown Node",
          color: 'green'
        })),
        emptyText: 'No gateways'
      }] : [])
    ],
    
    actions: [
      {
        label: 'Delete Subnet',
        icon: Trash2,
        color: 'text-gray-400',
        hoverColor: 'text-red-400',
        bgHover: 'hover:bg-red-900/20',
        onClick: () => onDelete(subnet)
      },
      {
        label: 'Edit Subnet',
        icon: Edit,
        color: 'text-gray-400',
        hoverColor: 'text-white',
        bgHover: 'hover:bg-gray-700',
        onClick: () => onEdit(subnet)
      }
    ]
  };
</script>

<GenericCard {...cardData} />