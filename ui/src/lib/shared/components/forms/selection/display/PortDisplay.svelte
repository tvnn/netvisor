<script lang="ts" context="module">
	import PortInlineEditor from './PortInlineEditor.svelte';

	export const PortDisplay: EntityDisplayComponent<Port> = {
		getId: (port: Port) => `${port.number}-${port.protocol}`,
		getLabel: (port: Port) => `Port ${port.number}`,
		getDescription: () => '',
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
	import type { Port } from '$lib/features/hosts/types/base';
	import type { EntityDisplayComponent } from '../types';
	import ListSelectItem from '../ListSelectItem.svelte';
	import { entities } from '$lib/shared/stores/metadata';

	export let item: Port;
</script>

<ListSelectItem {item} displayComponent={PortDisplay} />
