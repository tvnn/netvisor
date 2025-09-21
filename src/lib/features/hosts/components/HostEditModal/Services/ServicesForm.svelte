<script lang="ts">
  import { onMount } from 'svelte';
  import { get } from 'svelte/store';
  import ListConfigEditor from '$lib/shared/components/forms/selection/ListConfigEditor.svelte';
  import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
  import ServicesConfigPanel from './ServicesConfigPanel.svelte';
  import type { Port, Service } from '$lib/features/services/types/base';
  import type { Host } from '$lib/features/hosts/types/base';
  import { serviceTypes } from '$lib/shared/stores/metadata';
  import { createDefaultService, getServicesForHost } from '$lib/features/services/store';
  import type { TypeMetadata } from '$lib/shared/stores/metadata';
	import { ServiceDisplay } from '$lib/shared/components/forms/selection/display/ServiceDisplay.svelte';
	import { ServiceTypeDisplay } from '$lib/shared/components/forms/selection/display/ServiceTypeDisplay.svelte';
  
  export let form: any;
  export let formData: Host;
  
  // This will be bound to the parent HostEditor component
  export let currentServices: Service[] = [];
  
  let listConfigEditorRef: any;
  
  // Get current services for this host
  $: currentServices = getServicesForHost(formData.id);
    
  // Available service types for adding
  $: availableServiceTypes = serviceTypes.getItems()?.filter(service => 
    service.metadata?.can_be_added !== false
  ).sort((a, b) => a.category.localeCompare(b.category, 'en')) || [];
  
  // Event handlers
  function handleAddService(serviceTypeId: string) {    
    const serviceMetadata = serviceTypes.getItems()?.find(s => s.id === serviceTypeId);
    if (!serviceMetadata) return;
    
    const defaultPorts = serviceMetadata.metadata?.default_ports || [];
    
    const newService: Service = createDefaultService(serviceTypeId, formData.id, serviceTypes.getDisplay(serviceTypeId), defaultPorts)
    
    currentServices = [...currentServices, newService as Service];
  }
  
  function handleRemoveService(index: number) {    
    currentServices = currentServices.filter((_, i) => i !== index);
  }
  
  // Handle service changes - this will be called by ListConfigEditor through onChange prop
  function handleServiceChange(service: Service, index: number) {    
    if (index >= 0 && index < currentServices.length) {
      const updatedServices = [...currentServices];
      const oldService = updatedServices[index];
      
      updatedServices[index] = service;
      currentServices = updatedServices;
    } else {
      console.error('❌ ServicesForm: Invalid service index:', index);
    }
  }
</script>

<div class="space-y-6">  
  <ListConfigEditor
    {form}
    bind:items={currentServices}
    onChange={handleServiceChange}
    bind:this={listConfigEditorRef}
  >
    <svelte:fragment slot="list" let:items let:onEdit let:highlightedIndex>
      <ListManager
        label="Services"
        helpText="Services define what this host provides to the network."
        placeholder="Select service type to add..."
        emptyMessage="No services configured yet. Add one to get started."
        allowReorder={true}
        
        options={availableServiceTypes}
        {items}
        allowItemRemove={() => true}
        
        optionDisplayComponent={ServiceTypeDisplay}
        itemDisplayComponent={ServiceDisplay}
        
        onAdd={handleAddService}
        onRemove={handleRemoveService}
        {onEdit}
        {highlightedIndex}
      />
    </svelte:fragment>
    
    <svelte:fragment slot="config" let:selectedItem let:onChange>
      {#if selectedItem}
        <ServicesConfigPanel
          {form}
          service={selectedItem}
          onChange={(updatedService) => onChange(updatedService)}
          host_interfaces={formData.interfaces}
        />
      {/if}
    </svelte:fragment>
  </ListConfigEditor>
</div>