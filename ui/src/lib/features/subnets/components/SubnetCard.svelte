<script lang="ts">
	import { Edit, Trash2 } from 'lucide-svelte';
	import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
	import { entities, subnetTypes } from '$lib/shared/stores/metadata';
	import { formatServiceLabels } from '$lib/features/services/store';
	import { getSubnetServices, isContainerSubnet } from '../store';
	import type { Subnet } from '../types/base';

	export let subnet: Subnet;
	export let onEdit: (subnet: Subnet) => void = () => {};
	export let onDelete: (subnet: Subnet) => void = () => {};

	// $: dnsServices = getSubnetServices(subnet, 'is_dns_resolver');
	// $: gatewayServices = getSubnetServices(subnet, 'is_gateway');
	// $: reverseProxyServices = getSubnetServices(subnet, 'is_reverse_proxy');
	$: allServices = getSubnetServices(subnet);

	// $: dnsLabels = formatServiceLabels(dnsServices.map((s) => s.id));
	// $: gatewayLabels = formatServiceLabels(gatewayServices.map((s) => s.id));
	// $: reverseProxyLabels = formatServiceLabels(reverseProxyServices.map((s) => s.id));
	$: serviceLabels = formatServiceLabels(allServices.map((s) => s.id));

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
			// {
			// 	label: 'DNS Resolvers',
			// 	items: dnsLabels.map(({ id, label }) => ({
			// 		id,
			// 		label,
			// 		color: entities.getColorString('Dns')
			// 	})),
			// 	emptyText: 'No DNS resolvers'
			// },
			// {
			// 	label: 'Gateways',
			// 	items: gatewayLabels.map(({ id, label }) => ({
			// 		id,
			// 		label,
			// 		color: entities.getColorString('Gateway')
			// 	})),
			// 	emptyText: 'No gateways'
			// },
			// {
			// 	label: 'Reverse Proxies',
			// 	items: reverseProxyLabels.map(({ id, label }) => ({
			// 		id,
			// 		label,
			// 		color: entities.getColorString('ReverseProxy')
			// 	})),
			// 	emptyText: 'No reverse proxies'
			// },
			{
				label: 'Services',
				items: serviceLabels.map(({ id, label }) => ({
					id,
					label,
					color: entities.getColorString('Service')
				})),
				emptyText: 'No services'
			}
		],

		actions: [
			{
				label: 'Delete Subnet',
				icon: Trash2,
				color: 'btn-icon-danger',
				onClick: () => onDelete(subnet)
			},
			{
				label: 'Edit Subnet',
				icon: Edit,
				onClick: () => onEdit(subnet)
			}
		]
	};
</script>

<GenericCard {...cardData} />
