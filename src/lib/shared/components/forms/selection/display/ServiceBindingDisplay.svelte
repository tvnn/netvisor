<script lang="ts" context="module">
  import { Server } from 'lucide-svelte';
  import ServiceBindingInlineEditor from './ServiceBindingInlineEditor.svelte';
  import { entities, serviceTypes } from '$lib/shared/stores/metadata';
  import { getServiceHost, services } from '$lib/features/services/store';
  
  // We'll store the update handler globally so the inline editor can access it
  let currentBindingUpdateHandler: ((updates: Partial<ServiceBinding>) => void) | null = null;
  
  export const ServiceBindingDisplay: EntityDisplayComponent<ServiceBinding> = {
    getId: (binding: ServiceBinding) => `${binding.service_id}-${binding.interface_id}`,
    getLabel: (binding: ServiceBinding) => {
      const service = get(services).find(s => s.id === binding.service_id);
      return service?.name || 'Unknown Service';
    },
    getDescription: (binding: ServiceBinding) => {
      const host = getServiceHost(binding.service_id)
      
      return host ? host.name : "Unknown Host"
    },
    getIcon: () => Server,
    getIconColor: (binding: ServiceBinding) => {
      const service = get(services).find(s => s.id === binding.service_id);
      if (!service) return 'text-gray-400';
      
      const serviceType = serviceTypes.getItem(service.service_type);
      return serviceType ? `text-${serviceType.color}-400` : 'text-gray-400';
    },
    getTags: (binding: ServiceBinding) => {
      const service = get(services).find(s => s.id === binding.service_id);
      if (!service) return [];
      
      const tags = [];
      
      const serviceType = serviceTypes.getItem(service.service_type);

      const host = getServiceHost(binding.service_id)
      if (host) {
        const iface = host.interfaces.find(i => i.id == binding.interface_id);
        // Add interface indicator if we have the interface info
        if (iface) {
            tags.push({
            label: "Interface: " +  iface?.ip_address + (iface?.name ? " ("+iface?.name+")" : ""),
            color: entities.getColorHelper("Interface").string
            });
        }
      }
      
      return tags;
    },
    getIsDisabled: () => false,
    getCategory: (binding: ServiceBinding) => {
      const service = get(services).find(s => s.id === binding.service_id);
      if (!service) return null;
      
      const serviceType = serviceTypes.getItem(service.service_type);
      return serviceType?.category || null;
    },
    supportsInlineEdit: true,
    renderInlineEdit: (binding: ServiceBinding, onUpdate: (updates: Partial<ServiceBinding>) => void) => {
      // Store the update handler so the inline editor can access it
      currentBindingUpdateHandler = onUpdate;
      
      return {
        component: ServiceBindingInlineEditor,
        props: { 
          serviceBinding: binding, 
          onUpdate: currentBindingUpdateHandler
        }
      };
    }
  };
</script>

<script lang="ts">
  import type { ServiceBinding } from '$lib/features/groups/types/base';
  import type { DisplayComponentProps, EntityDisplayComponent } from '../types';
  import ListSelectItem from '../ListSelectItem.svelte';
	import { get } from 'svelte/store';
  
  type $Props = DisplayComponentProps<ServiceBinding>;
  
  export let item: ServiceBinding;
</script>

<ListSelectItem item={item} displayComponent={ServiceBindingDisplay} />