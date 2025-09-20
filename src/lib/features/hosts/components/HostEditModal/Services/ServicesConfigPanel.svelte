<!-- src/lib/features/hosts/components/HostEditModal/Services/ServicesConfigPanel.svelte -->
<script lang="ts">
  import { field } from 'svelte-forms';
  import { AlertCircle, AlertTriangle } from 'lucide-svelte';
  import type { Service, Port } from '$lib/features/services/types/base';
  import type { Interface } from '$lib/features/hosts/types/base';
  import { entities, registry, serviceTypes } from '$lib/shared/stores/registry';
  import Tag from '$lib/shared/components/data/Tag.svelte';
  import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
  import { PortDisplay } from '$lib/shared/components/forms/selection/display/PortDisplay.svelte';
	import { InterfaceDisplay } from '$lib/shared/components/forms/selection/display/InterfaceDisplay.svelte';
	import InlineWarning from '$lib/shared/components/feedback/InlineWarning.svelte';
  
  export let form: any;
  export let service: Service | null = null;
  export let host_interfaces: Interface[] = [];
  export let onChange: (updatedService: Service) => void = () => {};
  
  let serviceNameField: any;
  
  // Get service metadata from registry - use serviceTypes helper for consistency
  $: serviceMetadata = service ? serviceTypes.getItem(service.service_type) : null;
  
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
      `service_name_${service.service_type}`,
      service.name,
      [serviceNameValidator()]
    );
        
    // Register with parent form
    if (form) {
      form[`service_name_${service.service_type}`] = serviceNameField;
    }
  }
  
  // Update service when field values change
  $: if (service && serviceNameField && $serviceNameField) {
    const updatedService: Service = {
      ...service,
      name: $serviceNameField.value,
    };
    
    // Only trigger onChange if values actually changed
    if (updatedService.name !== service.name) {
      onChange(updatedService);
    }
  }

  function handleAddPort(portId: string) {
    if (!service) return;
    
    // Parse the port ID back to find the port
    // const [numberStr, protocol] = portId.split('-');
    // const number = parseInt(numberStr);
    
    // if (!selectedPort) return;
    
    // const updatedService = {
    //   ...service,
    //   ports: [...service.ports, selectedPort]
    // };
    
    // onChange(updatedService);
  }
  
  function handleRemovePort(index: number) {
    if (!service) return;
    
    const updatedService = {
      ...service,
      ports: service.ports.filter((_, i) => i !== index)
    };
    
    onChange(updatedService);
  }
  
  // Interface binding management
  function handleAddInterface(interfaceId: string) {
    if (!service) return;
        
    // Validate interface exists in host_interfaces
    const interfaceExists = host_interfaces.find(iface => iface.id === interfaceId);
    if (!interfaceExists) return;
    if (service.interface_bindings.includes(interfaceId)) return;
    
    const updatedService = {
      ...service,
      interface_bindings: [...service.interface_bindings, interfaceId]
    };
    
    onChange(updatedService);
  }
  
  function handleRemoveInterface(index: number) {
    if (!service) return;
    
    if (index < 0 || index >= service.interface_bindings.length) return;
    
    const removedInterfaceId = service.interface_bindings[index];
    const updatedService = {
      ...service,
      interface_bindings: service.interface_bindings.filter((_, i) => i !== index)
    };
        
    onChange(updatedService);
  }
  
  // Reactive statement for bound interfaces
  $: boundInterfaces = service ? 
    service.interface_bindings
      .map(id => {
        const iface = host_interfaces.find(iface => iface.id === id);
        if (!iface) {
          console.warn(`⚠️ Interface binding ${id} not found in host interfaces for service ${service.name}`);
        }
        return iface;
      })
      .filter(Boolean) as Interface[] : 
    [];
  
  // Reactive statement for available interfaces
  $: availableInterfaces = host_interfaces.filter(iface => {
    const isAlreadyBound = service?.interface_bindings.includes(iface.id) || false;
    return !isAlreadyBound;
  });
  
  let colorHelper = entities.getColorHelper("Service")
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
                   focus:outline-none focus:ring-2
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
    
    <!-- Interface Bindings -->
    {#if host_interfaces.length > 0}
      <div class="space-y-4">
        <ListManager
          label="Interface Bindings"
          helpText="Select which network interfaces this service is bound to"
          placeholder="Select an interface to bind..."
          emptyMessage="No interfaces bound to this service. Add one to get started."
          allowReorder={true}
          allowDuplicates={false}
          allowItemEdit={() => false}
          allowItemRemove={() => true}
          
          options={availableInterfaces}
          items={boundInterfaces}
          
          optionDisplayComponent={InterfaceDisplay}
          itemDisplayComponent={InterfaceDisplay}
          
          onAdd={handleAddInterface}
          onRemove={handleRemoveInterface}
          onEdit={() => {}}
        />

        <!-- Warning for no bindings -->
        {#if service.interface_bindings.length === 0}
          <InlineWarning title="No Interface Bindings" body="This service has no interface bindings, which may cause issues with visualization. Consider binding it to at least one interface." />
        {/if}
      </div>
    {:else}
      <InlineWarning title="No Interfaces Available" body="Add network interfaces to the host first, then bind services to specific interfaces." />
    {/if}
    
    <!-- Ports Configuration using ListManager -->
    <div class="space-y-4">
      <ListManager
        label="Ports"
        helpText="Configure which ports this service uses"
        placeholder="Select a port to add"
        allowDuplicates={false}
        allowItemEdit={() => true}
        allowItemRemove={() => true}
        allowReorder={false}
        
        options={[]}
        items={service.ports}
        
        optionDisplayComponent={PortDisplay}
        itemDisplayComponent={PortDisplay}
        
        onAdd={handleAddPort}
        onRemove={handleRemovePort}
        onEdit={() => {}}
      />
    </div>
  </div>
{/if}