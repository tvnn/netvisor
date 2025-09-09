<script lang="ts">
  import { field } from 'svelte-forms';
  import { required } from 'svelte-forms/validators';
  import { AlertCircle, AlertTriangle, CheckCircle } from 'lucide-svelte';
  import type { Service, Port, Endpoint } from '$lib/features/services/types/base';
  import { getServiceDisplayName, formatServicePorts } from '$lib/features/services/types/base';
  import { registry } from '$lib/shared/stores/registry';
	import Tag from '$lib/shared/components/data/Tag.svelte';
  import ListManager from '$lib/shared/components/forms/ListManager.svelte';
	import PortListItem from '$lib/shared/components/forms/PortListItem.svelte';
  
  export let form: any;
  export let service: Service | null = null;
  export let open_ports: Port[] = [];
  export let onChange: (updatedService: Service) => void = () => {};
  
  let serviceNameField: any;
  let confirmedField: any;
  
  // Get service metadata from registry
  $: serviceMetadata = service ? $registry?.services?.find(s => s.id === service.type) : null;
  
  // Validators
  const serviceNameValidator = () => (value: string) => {
    if (!value || value.trim().length === 0) {
      return { name: 'required', valid: false };
    }
    return { name: 'valid', valid: true };
  };
  
  // Initialize form fields when service changes
  $: if (service && serviceMetadata) {
    serviceNameField = field(
      `service_name_${service.type}`,
      service.name,
      [serviceNameValidator()]
    );
        
    // Register with parent form
    if (form) {
      form[`service_name_${service.type}`] = serviceNameField;
      form[`service_confirmed_${service.type}`] = confirmedField;
    }
  }
  
  // Update service when field values change
  $: if (service && serviceNameField && confirmedField && $serviceNameField && $confirmedField) {
    const updatedService: Service = {
      ...service,
      name: $serviceNameField.value,
    };
    
    // Only trigger onChange if values actually changed
    if (updatedService.name !== service.name) {
      onChange(updatedService);
    }
  }
  
  // Port management functions for ListManager
  function getPortId(port: Port): string {
    return `${port.number}-${port.protocol}`;
  }
  
  function getPortLabel(port: Port): string {
    return `Port ${port.number} (${port.protocol.toUpperCase()})`;
  }
  
  function getPortDescription(port: Port): string {
    return `${port.protocol.toUpperCase()} port ${port.number}`;
  }

  function handleAddPort(portId: string) {
    if (!service) return;
    
    // Find the port from open_ports by ID
    const selectedPort = open_ports.find(p => getPortId(p) === portId);
    if (!selectedPort) return;
    
    const updatedService = {
      ...service,
      ports: [...service.ports, selectedPort]
    };
    
    onChange(updatedService);
  }
  
  function handleUpdatePort(index: number, updates: Partial<Port>) {
    if (!service) return;
    
    const updatedPorts = [...service.ports];
    updatedPorts[index] = { ...updatedPorts[index], ...updates };
    
    const updatedService = {
      ...service,
      ports: updatedPorts
    };
    
    onChange(updatedService);
  }
  
  function handleRemovePort(index: number) {
    if (!service) return;
    
    const updatedService = {
      ...service,
      ports: service.ports.filter((_, i) => i !== index)
    };
    
    onChange(updatedService);
  }
  
</script>

{#if service && serviceMetadata}
  <div class="space-y-6">
      <!-- Service Info -->
      <div class="flex gap-3 items-center">
        <h3 class="text-lg font-medium text-white">{serviceMetadata.display_name}</h3>
        <Tag
          label={serviceMetadata.category}
          color={serviceMetadata.color}/>
      </div>
      
      <p class="text-sm text-gray-400">{serviceMetadata.description}</p>  
    <div class="border-b border-gray-600"></div>

    <!-- Basic Configuration -->
    <div class="space-y-4">
      
      <!-- Service Name Field -->
      {#if serviceNameField}
        <div class="space-y-2">
          <label for="service_name" class="block text-sm font-medium text-gray-300">
            Service Name <span class="text-red-400">*</span>
          </label>
          <input
            id="service_name"
            type="text"
            bind:value={$serviceNameField.value}
            class="w-full px-3 py-2 bg-gray-700 border rounded-md text-white 
                   focus:outline-none focus:ring-2 focus:ring-blue-500
                   {$serviceNameField.errors.length > 0 ? 'border-red-500' : 'border-gray-600'}"
            placeholder="Enter a descriptive name..."
          />
          {#if $serviceNameField.errors.length > 0}
            <div class="flex items-center gap-2 text-red-400">
              <AlertCircle size={16} />
              <p class="text-xs">{$serviceNameField.errors[0]}</p>
            </div>
          {/if}
          <p class="text-xs text-gray-400">
            Give this service a meaningful name like "Main Web Server" or "Internal API"
          </p>
        </div>
      {/if}
    </div>
    
    <!-- Ports Configuration using ListManager -->
    <div class="space-y-4">
      <ListManager
        label="Ports"
        helpText="Configure which ports this service uses"
        placeholder="Select a port to add"
        options={open_ports}
        items={service.ports}
        getOptionId={getPortId}
        getOptionLabel={getPortLabel}
        getOptionDescription={getPortDescription}
        getItemId={getPortId}
        getItemLabel={getPortLabel}
        getItemDescription={getPortDescription}
        onAdd={handleAddPort}
        onRemove={handleRemovePort}
        allowDuplicates={false}
        allowItemEdit={() => false}
        allowItemRemove={() => true}
        allowReorder={false}
      >
        <svelte:fragment slot="item" let:item let:index>
          <PortListItem 
            port={item} 
            onUpdate={(updates: Partial<Port>) => handleUpdatePort(index, updates)} 
          />
        </svelte:fragment>
      </ListManager>
    </div>
  </div>
{:else}
  <div class="flex-1 min-h-0 flex items-center justify-center text-gray-400">
    <div class="text-center">
      <div class="text-lg mb-2">No service selected</div>
      <div class="text-sm">Select a service from the list to configure it</div>
    </div>
  </div>
{/if}