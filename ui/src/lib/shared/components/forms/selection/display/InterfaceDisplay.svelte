<script lang="ts" context="module">
	import { Network } from 'lucide-svelte';
	import { isContainerSubnet, subnets } from '$lib/features/subnets/store';
	import { get } from 'svelte/store';

	// Helper function to find subnet by ID
	function findSubnetById(subnetId: string) {
		return get(subnets).find((s) => s.id === subnetId) || null;
	}

	export const InterfaceDisplay: EntityDisplayComponent<Interface> = {
		getId: (iface: Interface) => iface.id,
		getLabel: (iface: Interface) => (iface.name ? iface.name : 'Unnamed Interface'),
		getDescription: (iface: Interface) => {
			const parts = [iface.ip_address];
			if (iface.mac_address) {
				parts.push(iface.mac_address);
			} else {
				parts.push('No MAC');
			}
			return parts.join(' • ');
		},
		getIcon: () => entities.getIconComponent('Interface'),
		getIconColor: () => 'text-purple-400',
		getTags: (iface: Interface) => {
			const subnet = findSubnetById(iface.subnet_id);
			const tags = [];
			if (subnet && !isContainerSubnet(subnet.id)) {
				tags.push({
					label: subnet.cidr,
					color: entities.getColorHelper('Subnet').string
				});
			}
			return tags;
		},
		getIsDisabled: () => false,
		getCategory: () => null
	};
</script>

<script lang="ts">
	import ListSelectItem from '$lib/shared/components/forms/selection/ListSelectItem.svelte';
	import type { Interface } from '$lib/features/hosts/types/base';
	import type { DisplayComponentProps, EntityDisplayComponent } from '../types';
	import { entities } from '$lib/shared/stores/metadata';

	type $$Props = DisplayComponentProps<Interface>;

	export let item: Interface;
</script>

<ListSelectItem {item} displayComponent={InterfaceDisplay} />
