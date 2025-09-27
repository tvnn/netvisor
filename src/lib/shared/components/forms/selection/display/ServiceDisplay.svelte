<script lang="ts" context="module">
  import { serviceDefinitions } from '$lib/shared/stores/metadata';
  
  export const ServiceDisplay: EntityDisplayComponent<Service> = {
    getId: (service: Service) => service.id,
    getLabel: (service: Service) => service.name,
    getDescription: (service: Service) => {
      let binding_count = service.interface_bindings.length
      let port_count = service.port_bindings.length
      return port_count + " port" + (port_count==1?"":"s") + " · " + binding_count + " interface binding" + (binding_count==1?"":"s")
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
  
  type $$Props = DisplayComponentProps<Service>;
  
  export let item: Service;
</script>

<ListSelectItem item={item} displayComponent={ServiceDisplay} />