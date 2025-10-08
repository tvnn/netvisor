<script lang="ts" context="module">
	import type { Port } from '$lib/features/hosts/types/base';
	import type { EntityDisplayComponent } from '../types';
	import { entities } from '$lib/shared/stores/metadata';
	import { getServicesForPort } from '$lib/features/services/store';
	import PortInlineEditor from './PortInlineEditor.svelte';

	export const PortDisplay: EntityDisplayComponent<Port> = {
		getId: (port: Port) => `${port.number}-${port.protocol}`,
		getLabel: (port: Port) => `Port ${port.number}`,
		getDescription: (port: Port) => {
			let services = getServicesForPort(port.id);
			if (services.length > 0) {
				return services
					.map((s) => {
						let binding = s.bindings.find((b) => b.port_id == port.id);
						let iface = getInterfaceFromId(binding?.interface_id || '');
						if (iface) {
							return s.name + ' on ' + formatInterface(iface);
						} else {
							return s.name + ' on ' + 'Unknown Interface';
						}
					})
					.join(' • ');
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
		getCategory: () => null,
		supportsInlineEdit: true,
		renderInlineEdit: (port: Port, onUpdate: (updates: Partial<Port>) => void) => {
			return {
				component: PortInlineEditor,
				props: { port, onUpdate }
			};
		}
	};
</script>

<script lang="ts">
	import ListSelectItem from '../ListSelectItem.svelte';
	import { formatInterface, getInterfaceFromId } from '$lib/features/hosts/store';

	export let item: Port;
</script>

<ListSelectItem {item} displayComponent={PortDisplay} />
