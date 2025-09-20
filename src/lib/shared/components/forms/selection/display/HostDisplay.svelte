<script lang="ts" context="module">
  import { Network } from 'lucide-svelte';
  import type { Host } from '$lib/features/hosts/types/base';
  import { serviceTypes } from '$lib/shared/stores/registry';
  
  export const HostDisplay: EntityDisplayComponent<Host> = {
    getId: (host: Host) => host.id,
    getLabel: (host: Host) => host.name,
    getDescription: (host: Host) => getHostTargetString(host),
    getIcon: () => Network,
    getIconColor: () => 'text-blue-400',
    getTags: (host: Host) => {

      let services = get(getServicesForHost(host.id))

      return services.map(service => ({
        label: serviceTypes.getDisplay(service.service_type.type),
        color: serviceTypes.getColorString(service.service_type.type)
      }));
    },
    getIsDisabled: () => false,
    getCategory: () => null
  };
</script>

<script lang="ts">
	import { getServicesForHost } from '$lib/features/services/store';
	import { get } from 'svelte/store';
	import type { DisplayComponentProps, EntityDisplayComponent } from '../types';
	import ListSelectItem from '../ListSelectItem.svelte';
	import { getHostTargetString } from '$lib/features/hosts/store';
  
  type $$Props = DisplayComponentProps<Host>;
  
  export let item: Host;
</script>

<ListSelectItem item={item} displayComponent={HostDisplay} />