<script lang="ts" context="module">
  import type { EntityDisplayComponent } from '../types';
  
  // Simplified fallback display component for services without hosts
  export const ServiceAsHostDisplay: EntityDisplayComponent<Service> = {
    getId: (service: Service) => service.id,
    getLabel: (service: Service) => getServiceHost(service.id)?.name || "Unknown Host",
    getDescription: (service: Service) => {
        let host = getServiceHost(service.id) 
        return host ? getHostTargetString(host) || "Unknown Host" : "Unknown Host";
    },
    getIcon: () => entities.getIconComponent("Host"),
    getIconColor: () => entities.getColorHelper("Host").icon,
    getTags: (service: Service) => {
        let host = getServiceHost(service.id)
        if (host) {
            let services = getServicesForHost(host.id)

            return services.map(service => ({
                label: serviceDefinitions.getName(service.service_definition),
                color: serviceDefinitions.getColorString(service.service_definition)
            }));
        }
        return [];
    },
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

<!-- If we have a host, render it using HostDisplay, otherwise show fallback -->
{#if host}
  <ListSelectItem item={host} displayComponent={HostDisplay} />
{:else}
  <!-- Fallback for when host is not found -->
  <ListSelectItem item={item} displayComponent={ServiceAsHostDisplay} />
{/if}