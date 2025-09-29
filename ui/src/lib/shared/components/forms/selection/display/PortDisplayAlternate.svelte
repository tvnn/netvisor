<script lang="ts" context="module">
	import type { Port } from '$lib/features/hosts/types/base';
	import type { EntityDisplayComponent } from '../types';
	import { entities } from '$lib/shared/stores/metadata';
	import { getServicesForPort } from '$lib/features/services/store';
	import { PortDisplay } from './PortDisplay.svelte';

	export const PortDisplayAlternate: EntityDisplayComponent<Port> = {
		getId: (port: Port) => PortDisplay.getId(port),
		getLabel: (port: Port) => PortDisplay.getLabel(port),
		getDescription: (port: Port) => {
			let services = getServicesForPort(port.id);
			console.log(port);
			console.log(services);
			if (services.length > 0) {
				return services.map((s) => s.name).join(' • ');
			} else {
				return 'Unassigned';
			}
		},
		getIcon: () => entities.getIconComponent('Port'),
		getIconColor: () => entities.getColorHelper('Port').icon,
		getTags: (port: Port) => [
			{
				label: port.protocol.toUpperCase(),
				color: port.protocol === 'Tcp' ? 'blue' : 'purple'
			}
		],
		getIsDisabled: () => false,
		getCategory: () => null
	};
</script>

<script lang="ts">
	import ListSelectItem from '../ListSelectItem.svelte';

	export let item: Port;
</script>

<ListSelectItem {item} displayComponent={PortDisplayAlternate} />
