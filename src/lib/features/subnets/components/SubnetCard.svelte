<script lang="ts">
  import { Edit, Trash2, Network } from 'lucide-svelte';
	import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
	import { hosts } from '$lib/features/hosts/store';
	import { get } from 'svelte/store';
	import { subnetTypes } from '$lib/shared/stores/registry';
  
  export let subnet: Subnet;
  export let onEdit: (subnet: Subnet) => void = () => {};
  export let onDelete: (subnet: Subnet) => void = () => {};
    
  function getHostName(id: string): string | null {
    return get(hosts).find(h => h.id == id)?.name || null
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
          label: subnetTypes.getDisplay(subnet.subnet_type),
          color: subnetTypes.getColorString(subnet.subnet_type)
        }],
        emptyText: 'No type specified'
      },
      ...(subnet.dns_resolvers && subnet.dns_resolvers.length > 0 ? [{
        label: 'DNS Resolvers',
        items: subnet.dns_resolvers.map((resolverId) => ({
          id: resolverId,
          label: getHostName(resolverId) || "Unknown Host",
          color: 'yellow'
        })),
        emptyText: 'No DNS resolvers'
      }] : []),
      ...(subnet.gateways && subnet.gateways.length > 0 ? [{
        label: 'Gateways',
        items: subnet.gateways.map((gatewayId) => ({
          id: gatewayId,
          label: getHostName(gatewayId) || "Unknown Host",
          color: 'green'
        })),
        emptyText: 'No gateways'
      }] : []),
      ...(subnet.reverse_proxies && subnet.reverse_proxies.length > 0 ? [{
        label: 'Reverse Proxies',
        items: subnet.reverse_proxies.map((rproxyId) => ({
          id: rproxyId,
          label: getHostName(rproxyId) || "Unknown Host",
          color: 'emerald'
        })),
        emptyText: 'No reverse proxies'
      }] : []),
      ...(subnet.hosts && subnet.hosts.length > 0 ? [{
        label: 'Hosts',
        items: subnet.hosts.map((hostId) => ({
          id: hostId,
          label: getHostName(hostId) || "Unknown Host",
          color: 'blue'
        })),
        emptyText: 'No hosts'
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