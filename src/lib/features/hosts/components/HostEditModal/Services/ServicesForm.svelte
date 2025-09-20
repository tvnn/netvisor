<script lang="ts">
  import { onMount } from 'svelte';
  import { get } from 'svelte/store';
  import ListConfigEditor from '$lib/shared/components/forms/selection/ListConfigEditor.svelte';
  import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
  import ServicesConfigPanel from './ServicesConfigPanel.svelte';
  import type { Port, Service } from '$lib/features/services/types/base';
  import type { Host } from '$lib/features/hosts/types/base';
  import { serviceTypes } from '$lib/shared/stores/registry';
  import { createDefaultService, getServicesForHost, updateHostServices } from '$lib/features/services/store';
  import type { TypeMetadata } from '$lib/shared/stores/registry';
	import { ServiceDisplay } from '$lib/shared/components/forms/selection/display/ServiceDisplay.svelte';
	import { ServiceTypeDisplay } from '$lib/shared/components/forms/selection/display/ServiceTypeDisplay.svelte';
  
  export let form: any;
  export let formData: Host;
  
  // Local state for managing services during editing
  let currentServices: Service[] = [];
  let servicesToDelete: string[] = [];
  
  // Get current services for this host
  $: hostServicesStore = getServicesForHost(formData.id);
  
  // Initialize services when component mounts or host changes
  $: if ($hostServicesStore) {
    currentServices = [...$hostServicesStore];
  }
  
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
  
  function handleServiceChange(service: Service, index: number) {
    if (index >= 0 && index < currentServices.length) {
      const updatedServices = [...currentServices];
      updatedServices[index] = service;
      currentServices = updatedServices;
    }
  }
  
  function handleRemoveService(index: number) {
    const serviceToRemove = currentServices[index];
    
    // If service has an ID, mark it for deletion
    if (serviceToRemove.id) {
      servicesToDelete = [...servicesToDelete, serviceToRemove.id];
    }
    
    currentServices = currentServices.filter((_, i) => i !== index);
  }
  
  // Save changes when parent form is submitted
  export async function saveServices() {
    await updateHostServices(formData.id, currentServices, servicesToDelete);
    servicesToDelete = [];
  }
</script>

<div class="space-y-6">  
  <ListConfigEditor
    {form}
    bind:items={currentServices}
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
          open_ports={formData.open_ports}
          host_interfaces={formData.interfaces}
        />
      {/if}
    </svelte:fragment>
  </ListConfigEditor>
</div>