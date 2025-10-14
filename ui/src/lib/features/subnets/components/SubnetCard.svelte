<script lang="ts">
	import { Edit, Trash2 } from 'lucide-svelte';
	import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
	import { entities, subnetTypes } from '$lib/shared/stores/metadata';
	import { formatServiceAsHost } from '$lib/features/services/store';
	import { getSubnetServices, isContainerSubnet } from '../store';
	import type { Subnet } from '../types/base';

	export let subnet: Subnet;
	export let onEdit: (subnet: Subnet) => void = () => {};
	export let onDelete: (subnet: Subnet) => void = () => {};

	// Build card data
	$: cardData = {
		title: subnet.name,
		subtitle: isContainerSubnet(subnet.id) ? '' : subnet.cidr,
		iconColor: subnetTypes.getColorHelper(subnet.subnet_type).icon,
		icon: subnetTypes.getIconComponent(subnet.subnet_type),

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
				items: getSubnetServices(subnet, 'is_dns_resolver').map((s) => ({
					id: s.id,
					label: formatServiceAsHost(s.id),
					color: entities.getColorString('Dns')
				})),
				emptyText: 'No DNS resolvers'
			},
			{
				label: 'Gateways',
				items: getSubnetServices(subnet, 'is_gateway').map((s) => ({
					id: s.id,
					label: formatServiceAsHost(s.id),
					color: entities.getColorString('Gateway')
				})),
				emptyText: 'No gateways'
			},
			{
				label: 'Reverse Proxies',
				items: getSubnetServices(subnet, 'is_reverse_proxy').map((s) => ({
					id: s.id,
					label: formatServiceAsHost(s.id),
					color: entities.getColorString('ReverseProxy')
				})),
				emptyText: 'No reverse proxies'
			},
			{
				label: 'Services',
				items: getSubnetServices(subnet).map((s) => ({
					id: s.id,
					label: formatServiceAsHost(s.id),
					color: entities.getColorString('Service')
				})),
				emptyText: 'No reverse proxies'
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
