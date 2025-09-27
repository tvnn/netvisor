<script lang="ts" context="module">
  import { Server } from 'lucide-svelte';
  import ServiceBindingInlineEditor from './ServiceBindingInlineEditor.svelte';
  import { entities, serviceDefinitions } from '$lib/shared/stores/metadata';
  import { getServiceHost, services } from '$lib/features/services/store';
    
  export const ServiceBindingDisplay: EntityDisplayComponent<ServiceBinding> = {
    getId: (binding: ServiceBinding) => serviceBindingToId(binding),
    getLabel: (binding: ServiceBinding) => {
      const service = get(services).find(s => s.id === binding.service_id);
      return service?.name || 'Unknown Service';
    },
    getDescription: (binding: ServiceBinding) => {
      const host = getServiceHost(binding.service_id)
      
      return host ? host.name : "Unknown Host"
    },
    getIcon: (binding: ServiceBinding) => {
      const service = get(services).find(s => s.id === binding.service_id);
      if (!service) return entities.getIconComponent("Service");

      return serviceDefinitions.getIconComponent(service.service_definition)
    },
    getIconColor: (binding: ServiceBinding) => {
      const service = get(services).find(s => s.id === binding.service_id);
      if (!service) return 'text-gray-400';
      
      return serviceDefinitions.getColorHelper(service.service_definition).icon;
    },
    getTags: (binding: ServiceBinding) => {
      const service = get(services).find(s => s.id === binding.service_id);
      if (!service) return [];
      
      const tags = [];
      
      const iface = getInterfaceFromId(binding.interface_id)

      if (iface) {
          tags.push({
          label: (iface?.name ? iface?.name+": " : "") + iface?.ip_address,
          color: entities.getColorHelper("Interface").string
          });
      }

      const port = getPortFromId(binding.port_id)

      if (port) {
          tags.push({
          label: formatPort(port),
          color: entities.getColorHelper("Port").string
          });
      }
      
      return tags;
    },
    getIsDisabled: () => false,
    getCategory: (binding: ServiceBinding) => {
      const service = get(services).find(s => s.id === binding.service_id);
      if (!service) return null;
      
      const serviceType = serviceDefinitions.getItem(service.service_definition);
      return serviceType?.category || null;
    },
    supportsInlineEdit: true,
    renderInlineEdit: (binding: ServiceBinding, onUpdate: (updates: Partial<ServiceBinding>) => void) => {      
      return {
        component: ServiceBindingInlineEditor,
        props: { 
          serviceBinding: binding, 
          onUpdate
        }
      };
    }
  };
</script>

<script lang="ts">
    import type { ServiceBinding } from "$lib/features/hosts/types/base";
  import type { DisplayComponentProps, EntityDisplayComponent } from '../types';
  import ListSelectItem from '../ListSelectItem.svelte';
	import { get } from 'svelte/store';
	import type { FormApi } from '../../types';
	import { getInterfaceFromId, getPortFromId, serviceBindingToId } from '$lib/features/hosts/store';
	import { formatPort } from '$lib/shared/utils/formatting';
  
  type $Props = DisplayComponentProps<ServiceBinding>;
  
  export let item: ServiceBinding;
</script>

<ListSelectItem item={item} displayComponent={ServiceBindingDisplay} />