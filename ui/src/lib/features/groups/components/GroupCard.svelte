<script lang="ts">
	import { Edit, Trash2 } from 'lucide-svelte';
	import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
	import type { Group } from '../types/base';
	import { entities, groupTypes } from '$lib/shared/stores/metadata';
	import { formatServiceLabels, getServicesForGroupReactive } from '$lib/features/services/store';

	export let group: Group;
	export let onEdit: (group: Group) => void = () => {};
	export let onDelete: (group: Group) => void = () => {};

	$: groupServicesStore = getServicesForGroupReactive(group.id);
	$: groupServices = $groupServicesStore;

	$: groupServiceLabels = formatServiceLabels(groupServices.map((s) => s.id));

	// Build card data
	$: cardData = {
		title: group.name,
		iconColor: groupTypes.getColorHelper(group.group_type).icon,
		icon: groupTypes.getIconComponent(group.group_type),

		sections: group.description
			? [
					{
						label: 'Description',
						value: group.description
					}
				]
			: [],

		lists: [
			{
				label: 'Group Type',
				items: [
					{
						id: 'type',
						label: groupTypes.getName(group.group_type),
						color: groupTypes.getColorString(group.group_type)
					}
				],
				emptyText: 'No type specified'
			},
			{
				label: 'Services',
				items: groupServiceLabels.map(({ id, label }, i) => {
					return {
						id: id + i,
						label,
						color: entities.getColorString('Service')
					};
				}),
				emptyText: 'No services in group'
			}
		],

		actions: [
			{
				label: 'Delete Group',
				icon: Trash2,
				class: 'btn-icon-danger',
				onClick: () => onDelete(group)
			},
			{
				label: 'Edit Group',
				icon: Edit,
				onClick: () => onEdit(group)
			}
		]
	};
</script>

<GenericCard {...cardData} />
