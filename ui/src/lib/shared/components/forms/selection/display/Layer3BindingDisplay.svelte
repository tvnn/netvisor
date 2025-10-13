<script lang="ts" context="module">
	import { entities, serviceDefinitions } from '$lib/shared/stores/metadata';
	import { getServiceForBinding } from '$lib/features/services/store';

	export const Layer3BindingDisplay: EntityDisplayComponent<Layer3Binding> = {
		getId: (binding: Layer3Binding) => binding.id,
		getLabel: (binding: Layer3Binding) => {
			const iface = getInterfaceFromId(binding.interface_id);
			const interfaceFormatted = iface ? formatInterface(iface) : 'Unknown Interface';
			return interfaceFormatted;
		},
		getDescription: () => '',
		getIcon: () => Link2,
		getIconColor: () => entities.getColorHelper('Interface').icon,
		getTags: () => [],
		getIsDisabled: () => false,
		getCategory: (binding: Layer3Binding) => {
			const service = getServiceForBinding(binding.id);
			if (!service) return null;

			const serviceType = serviceDefinitions.getItem(service.service_definition);
			return serviceType?.category || null;
		},
		supportsInlineEdit: true,
		renderInlineEdit: (
			binding: Layer3Binding,
			onUpdate: (updates: Partial<Layer3Binding>) => void,
			context: { service?: Service; host?: Host }
		) => {
			return {
				component: Layer3BindingInlineEditor,
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
	import { formatInterface, getInterfaceFromId } from '$lib/features/hosts/store';
	import type { Layer3Binding, Service } from '$lib/features/services/types/base';
	import { Link2 } from 'lucide-svelte';
	import type { Host } from '$lib/features/hosts/types/base';
	import Layer3BindingInlineEditor from './Layer3BindingInlineEditor.svelte';

	export let item: Layer3Binding;
</script>

<ListSelectItem {item} displayComponent={Layer3BindingDisplay} />
