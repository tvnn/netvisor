<script lang="ts">
	import { Edit, Trash2, Users } from 'lucide-svelte';
	import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
	import { hosts } from '$lib/features/hosts/store';
	import { get } from 'svelte/store';
	import type { Group } from '../types/base';
	import { entities } from '$lib/shared/stores/metadata';
	import { getServiceById } from '$lib/features/services/store';

	export let group: Group;
	export let onEdit: (group: Group) => void = () => {};
	export let onDelete: (group: Group) => void = () => {};

	// Build card data
	$: cardData = {
		title: group.name,
		iconColor: entities.getColorHelper('Group').icon,
		icon: entities.getIconComponent('Group'),

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
				label: 'Services',
				items: group.service_bindings.map((s) => ({
					id: s.service_id,
					label: getServiceById(s.service_id)?.name || 'Unknown Service',
					color: entities.getColorString('Service')
				})),
				emptyText: 'No services in group'
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
