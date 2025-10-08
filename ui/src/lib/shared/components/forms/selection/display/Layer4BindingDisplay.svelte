<script lang="ts" context="module">
	import { entities, serviceDefinitions } from '$lib/shared/stores/metadata';
	import { getServiceForBinding } from '$lib/features/services/store';

	export const Layer4BindingDisplay: EntityDisplayComponent<Layer4Binding> = {
		getId: (binding: Layer4Binding) => binding.id,
		getLabel: (binding: Layer4Binding) => {
			const port = getPortFromId(binding.port_id);
			const iface = getInterfaceFromId(binding.interface_id);
			const portFormatted = port ? formatPort(port) : 'Unknown Port';
			const interfaceFormatted = iface ? formatInterface(iface) : 'Unknown Interface';
			return interfaceFormatted + ' · ' + portFormatted;
		},
		getDescription: () => '',
		getIcon: () => Link2,
		getIconColor: () => entities.getColorHelper('Port').icon,
		getTags: () => [],
		getIsDisabled: () => false,
		getCategory: (binding: Layer4Binding) => {
			const service = getServiceForBinding(binding);
			if (!service) return null;

			const serviceType = serviceDefinitions.getItem(service.service_definition);
			return serviceType?.category || null;
		},
		supportsInlineEdit: true,
		renderInlineEdit: (
			binding: Layer4Binding,
			onUpdate: (updates: Partial<Layer4Binding>) => void,
			context: { service?: Service; host?: Host }
		) => {
			return {
				component: Layer4BindingInlineEditor,
				props: {
					binding,
					onUpdate,
					service: context?.service,
					host: context?.host
				}
			};
		}
	};
</script>

<script lang="ts">
	import type { EntityDisplayComponent } from '../types';
	import ListSelectItem from '../ListSelectItem.svelte';
	import { formatInterface, getInterfaceFromId, getPortFromId } from '$lib/features/hosts/store';
	import { formatPort } from '$lib/shared/utils/formatting';
	import type { Layer4Binding, Service } from '$lib/features/services/types/base';
	import { Link2 } from 'lucide-svelte';
	import type { Host } from '$lib/features/hosts/types/base';
	import Layer4BindingInlineEditor from './Layer4BindingInlineEditor.svelte';

	export let item: Layer4Binding;
</script>

<ListSelectItem {item} displayComponent={Layer4BindingDisplay} />
