<script lang="ts" context="module">
  import { serviceDefinitions } from '$lib/shared/stores/metadata';
  
  export const ServiceDisplay: EntityDisplayComponent<Service> = {
    getId: (service: Service) => service.id,
    getLabel: (service: Service) => service.name,
    getDescription: (service: Service) => {
      let binding_count = service.interface_bindings.length
      return formatServicePorts(service.ports) + " · " + binding_count + " interface " + (binding_count == 1 ? "binding" : "bindings")
    },
    getIcon: (service: Service) => serviceDefinitions.getIconComponent(service.service_definition),
    getIconColor: (service: Service) => serviceDefinitions.getColorHelper(service.service_definition).icon,
    getTags: (service: Service) => [],
    getIsDisabled: () => false,
    getCategory: () => null,
  };
</script>

<script lang="ts">
  import ListSelectItem from '$lib/shared/components/forms/selection/ListSelectItem.svelte';
	import type { DisplayComponentProps, EntityDisplayComponent } from '../types';
	import type { Service } from '$lib/features/services/types/base';
	import type { TagProps } from '$lib/shared/components/data/types';
	import { formatServicePorts } from '$lib/features/services/store';
	import ServiceBindingInterfaceEditor from './ServiceBindingInlineEditor.svelte';
	import { getInterfaceFromId } from '$lib/features/hosts/store';
  
  type $$Props = DisplayComponentProps<Service>;
  
  export let item: Service;
</script>

<ListSelectItem item={item} displayComponent={ServiceDisplay} />