<script lang="ts" context="module">
	import { entities, serviceDefinitions } from '$lib/shared/stores/metadata';
	import { getServiceForBinding } from '$lib/features/services/store';

	export const PortInterfaceBindingDisplay: EntityDisplayComponent<PortInterfaceBinding> = {
		getId: (binding: PortInterfaceBinding) => binding.id,
		getLabel: (binding: PortInterfaceBinding) => {
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
		getCategory: (binding: PortInterfaceBinding) => {
			const service = getServiceForBinding(binding);
			if (!service) return null;

			const serviceType = serviceDefinitions.getItem(service.service_definition);
			return serviceType?.category || null;
		},
		supportsInlineEdit: true,
		renderInlineEdit: (
			binding: PortInterfaceBinding,
			onUpdate: (updates: Partial<PortInterfaceBinding>) => void,
			context: { service?: Service; host?: Host }
		) => {
			return {
				component: PortInterfaceBindingInlineEditor,
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
	import type { PortInterfaceBinding, Service } from '$lib/features/services/types/base';
	import PortInterfaceBindingInlineEditor from './PortInterfaceBindingInlineEditor.svelte';
	import { Link2 } from 'lucide-svelte';
	import type { Host } from '$lib/features/hosts/types/base';

	export let item: PortInterfaceBinding;
</script>

<ListSelectItem {item} displayComponent={PortInterfaceBindingDisplay} />
