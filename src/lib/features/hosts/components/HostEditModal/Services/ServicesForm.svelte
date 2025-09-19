<script lang="ts">
  import ListConfigEditor from '$lib/shared/components/forms/ListConfigEditor.svelte';
  import ServicesConfigPanel from './ServicesConfigPanel.svelte';
  import { createStyle } from '$lib/shared/utils/styling';
  import type { Port, Service } from '$lib/features/services/types/base';
  import { createDefaultService, formatServicePorts } from '$lib/features/services/types/base';
  import type { Host } from '$lib/features/hosts/types/base';
  import { registry, serviceTypes } from '$lib/shared/stores/registry';
  import type { TypeMetadata } from '$lib/shared/stores/registry';
	import type { TagProps } from '$lib/shared/components/data/types';
  
  export let form: any;
  export let formData: Host;
  
  // Computed values
  $: hostServices = formData.services || [];
  $: availableServiceTypes = serviceTypes.getItems().filter(service => 
    service.metadata?.can_be_added !== false
  ).sort((a, b) => a.category.localeCompare(b.category, 'en')) || [];
  
  // Event handlers
  function handleAddService(serviceTypeId: string) {
    const serviceMetadata = serviceTypes.getItems().find(s => s.id === serviceTypeId);
    if (!serviceMetadata) return;
    
    const defaultPorts = serviceMetadata.metadata?.default_ports || [];
    const defaultEndpoints = serviceMetadata.metadata?.default_endpoints || [];
    
    // const newService = createDefaultService(
    //   formData.id  
    //   serviceTypeId, 
    //   serviceMetadata.display_name,
    //   defaultPorts,
    //   defaultEndpoints
    // );
    
    // formData.services = [...hostServices, newService];
  }
  
  function handleServiceChange(service: Service, index: number) {
    if (index >= 0 && index < hostServices.length) {
      const updatedServices = [...hostServices];
      updatedServices[index] = service;
      formData.services = updatedServices;
    }
  }
  
  function handleRemoveService(index: number) {
    formData.services = hostServices.filter((_, i) => i !== index);
  }
  
  // Display functions for options (available service types)
  function getOptionId(serviceMetadata: TypeMetadata): string {
    return serviceMetadata.id;
  }

  function getOptionCategory(serviceMetadata: TypeMetadata): string {
    return serviceMetadata.category;
  }
  
  function getOptionLabel(serviceMetadata: TypeMetadata): string {
    return serviceMetadata.display_name;
  }
  
  function getOptionDescription(serviceMetadata: TypeMetadata): string {
    return serviceMetadata.description;
  }
  
  function getOptionIcon(serviceMetadata: TypeMetadata) {
    return createStyle(null, serviceMetadata.icon).IconComponent;
  }
  
  function getOptionIconColor(serviceMetadata: TypeMetadata) {
    return createStyle(serviceMetadata.color, null).colors.icon;
  }
  
  function getOptionTags(serviceMetadata: TypeMetadata) {
    const tags = [];
    // const defaultPorts = serviceMetadata.metadata?.default_ports || [];
    // if (defaultPorts.length > 0) {
    //   const portTags = defaultPorts.map((p: Port) => {
    //     return {
    //       label: `${p.number}${p.tcp && p.udp ? '/tcp+udp' : p.tcp ? '/tcp' : '/udp'}`,
    //       color:"blue"
    //     }}
    //   );
    //   tags.push(...portTags)
    // }
    
    return [];
  }
  
  // Display functions for items (current services)
  function getItemId(service: Service): string {
    return `${service.service_type.type}_${service.name}`;
  }
  
  function getItemLabel(service: Service): string {
    return service.name;
  }
  
  function getItemDescription(service: Service): string {    
    return [`${formatServicePorts(service.ports)}`].filter(Boolean).join(' • ');
  }
  
  function getItemIcon(service: Service) {
    return serviceTypes.getIcon(service.service_type.type)
  }
  
  function getItemIconColor(service: Service) {
    return serviceTypes.getColorHelper(service.service_type.type).icon
  }
  
  function getItemTags(service: Service) {
    const tags: TagProps[] = [];
    const serviceMetadata = serviceTypes.getItems().find(s => s.id === service.service_type.type);
    
    // if (serviceMetadata) {
    //   tags.push({
    //     label: serviceMetadata.category,
    //     color: serviceMetadata.color
    //   });
    // }
        
    return tags;
  }
</script>

<ListConfigEditor
  {form}
  bind:items={formData.services}
  options={availableServiceTypes}
  label="Services"
  helpText="Configure services running on this host"
  emptyMessage="No services configured. Add one to get started."
  
  allowReorder={true}
  placeholder="Select service type to add..."
  allowItemRemove={(item) => serviceTypes.getMetadata(item.service_type.type).can_be_added}
  
  {getOptionId}
  {getOptionLabel}
  {getOptionDescription}
  {getOptionIcon}
  {getOptionIconColor}
  {getOptionTags}
  {getOptionCategory}
  
  {getItemId}
  {getItemLabel}
  {getItemDescription}
  {getItemIcon}
  {getItemIconColor}
  {getItemTags}
  
  onAdd={handleAddService}
  onRemove={handleRemoveService}
  onChange={handleServiceChange}
>
  <ServicesConfigPanel
    slot="config"
    let:selectedItem
    let:selectedIndex
    let:onChange
    
    {form}
    service={selectedItem}
    onChange={(updatedService) => onChange(updatedService)}
    open_ports={formData.open_ports}
  />
</ListConfigEditor>