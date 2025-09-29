<script lang="ts">
	import { Edit, Trash2 } from 'lucide-svelte';
	import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
	import { hosts } from '$lib/features/hosts/store';
	import { get } from 'svelte/store';
	import { entities, subnetTypes } from '$lib/shared/stores/metadata';
	import { getServiceHost } from '$lib/features/services/store';
	import { isContainerSubnet } from '../store';

	export let subnet: Subnet;
	export let onEdit: (subnet: Subnet) => void = () => {};
	export let onDelete: (subnet: Subnet) => void = () => {};

	function getHostName(id: string): string | null {
		return get(hosts).find((h) => h.id == id)?.name || null;
	}

	// Build card data
	$: cardData = {
		title: subnet.name,
		subtitle: isContainerSubnet(subnet.id) ? '' : subnet.cidr,
		iconColor: entities.getColorHelper('Subnet').icon,
		icon: entities.getIconComponent('Subnet'),

		sections: subnet.description
			? [
					{
						label: 'Description',
						value: subnet.description
					}
				]
			: [],

		lists: [
			{
				label: 'Network Type',
				items: [
					{
						id: 'type',
						label: subnetTypes.getName(subnet.subnet_type),
						color: subnetTypes.getColorString(subnet.subnet_type)
					}
				],
				emptyText: 'No type specified'
			},
			{
				label: 'DNS Resolvers',
				items: subnet.dns_resolvers.map((resolverId) => ({
					id: resolverId,
					label: getServiceHost(resolverId)?.name || 'Unknown Host',
					color: entities.getColorString('Dns')
				})),
				emptyText: 'No DNS resolvers'
			},
			{
				label: 'Gateways',
				items: subnet.gateways.map((gatewayId) => ({
					id: gatewayId,
					label: getServiceHost(gatewayId)?.name || 'Unknown Host',
					color: entities.getColorString('Gateway')
				})),
				emptyText: 'No gateways'
			},
			{
				label: 'Reverse Proxies',
				items: subnet.reverse_proxies.map((rproxyId) => ({
					id: rproxyId,
					label: getServiceHost(rproxyId)?.name || 'Unknown Host',
					color: entities.getColorString('ReverseProxy')
				})),
				emptyText: 'No reverse proxies'
			},
			{
				label: 'Hosts',
				items: subnet.hosts.map((hostId) => ({
					id: hostId,
					label: getHostName(hostId) || 'Unknown Host',
					color: entities.getColorString('Host')
				})),
				emptyText: 'No hosts'
			}
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
