<script lang="ts" context="module">
	export const SubnetDisplay: EntityDisplayComponent<Subnet> = {
		getId: (subnet: Subnet) => subnet.id,
		getLabel: (subnet: Subnet) => subnet.name,
		getDescription: (subnet: Subnet) => (subnet.name == subnet.cidr ? '' : subnet.cidr),
		getIcon: () => entities.getIconComponent('Subnet'),
		getIconColor: () => entities.getColorHelper('Subnet').icon,
		getTags: (_subnet: Subnet) => [],
		getIsDisabled: () => false,
		getCategory: () => null
	};
</script>

<script lang="ts">
	import ListSelectItem from '$lib/shared/components/forms/selection/ListSelectItem.svelte';
	import type { EntityDisplayComponent } from '../types';
	import { entities } from '$lib/shared/stores/metadata';

	export let item: Subnet;
</script>

<ListSelectItem {item} displayComponent={SubnetDisplay} />
