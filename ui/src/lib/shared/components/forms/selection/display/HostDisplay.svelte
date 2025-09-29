<script lang="ts" context="module">
	import type { Host } from '$lib/features/hosts/types/base';
	import { entities, serviceDefinitions } from '$lib/shared/stores/metadata';

	export const HostDisplay: EntityDisplayComponent<Host> = {
		getId: (host: Host) => host.id,
		getLabel: (host: Host) => host.name,
		getDescription: (host: Host) => getHostTargetString(host) || 'Unknown Host',
		getIcon: () => entities.getIconComponent('Host'),
		getIconColor: () => entities.getColorHelper('Host').icon,
		getTags: (host: Host) => {
			let services = getServicesForHost(host.id);

			return services.map((service) => ({
				label: serviceDefinitions.getName(service.service_definition),
				color: serviceDefinitions.getColorString(service.service_definition)
			}));
		},
		getIsDisabled: () => false,
		getCategory: () => null
	};
</script>

<script lang="ts">
	import { getServicesForHost } from '$lib/features/services/store';
	import type { EntityDisplayComponent } from '../types';
	import ListSelectItem from '../ListSelectItem.svelte';
	import { getHostTargetString } from '$lib/features/hosts/store';

	export let item: Host;
</script>

<ListSelectItem {item} displayComponent={HostDisplay} />
