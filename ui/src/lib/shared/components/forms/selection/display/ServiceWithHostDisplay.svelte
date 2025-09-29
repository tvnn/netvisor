<script lang="ts" context="module">
  import type { EntityDisplayComponent } from '../types';
  
  export const ServiceWithHostDisplay: EntityDisplayComponent<Service> = {
    getId: (service: Service) => service.id,
    getLabel: (service: Service) => service.name,
    getDescription: (service: Service) => {
        let host = getServiceHost(service.id) 
        
        return host ? host.name + " • " + getHostTargetString(host) || "Unknown Host" : "Unknown Host";
    },
    getIcon: (service: Service) => serviceDefinitions.getIconComponent(service.service_definition),
    getIconColor: (service: Service) => serviceDefinitions.getColorHelper(service.service_definition).icon,
    getTags: (service: Service) => [
      // {
      //   label: service.
      // }
    ],
    getIsDisabled: () => false,
    getCategory: () => null
  };
</script>

<script lang="ts">
  import { Network } from 'lucide-svelte';
  import type { DisplayComponentProps } from '../types';
  import type { Service } from '$lib/features/services/types/base';
  import { getServiceHost, getServicesForHost } from '$lib/features/services/store';
  import { HostDisplay } from './HostDisplay.svelte';
  import ListSelectItem from '../ListSelectItem.svelte';
	import { entities, serviceDefinitions } from '$lib/shared/stores/metadata';
	import { getHostTargetString } from '$lib/features/hosts/store';
  
  type $$Props = DisplayComponentProps<Service>;
  
  export let item: Service;
  
  // Get the host for this service
  $: host = getServiceHost(item.id);
</script>

<ListSelectItem item={item} displayComponent={ServiceWithHostDisplay} />