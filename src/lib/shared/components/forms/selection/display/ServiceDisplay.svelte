<script lang="ts" context="module">
  import { serviceTypes } from '$lib/shared/stores/metadata';
  
  export const ServiceDisplay: EntityDisplayComponent<Service> = {
    getId: (service: Service) => service.id,
    getLabel: (service: Service) => service.name,
    getDescription: (service: Service) => serviceTypes.getDisplay(service.service_type),
    getIcon: (service: Service) => serviceTypes.getIconComponent(service.service_type),
    getIconColor: (service: Service) => serviceTypes.getColorHelper(service.service_type).icon,
    getTags: (service: Service) => {

      const tags: TagProps[] = [];
            
      // Ports
      if (service.ports?.length > 0) {
        tags.push({
          label: formatServicePorts(service.ports),
          color: 'blue'
        });
      }
      
      return tags;
    },
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
  
  type $$Props = DisplayComponentProps<Service>;
  
  export let item: Service;
</script>

<ListSelectItem item={item} displayComponent={ServiceDisplay} />