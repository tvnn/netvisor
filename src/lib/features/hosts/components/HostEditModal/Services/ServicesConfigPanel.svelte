<!-- src/lib/features/hosts/components/HostEditModal/Services/ServicesConfigPanel.svelte -->
<script lang="ts">
  import { field } from 'svelte-forms';
  import { AlertCircle, AlertTriangle } from 'lucide-svelte';
  import type { Service, Port } from '$lib/features/services/types/base';
  import type { Interface } from '$lib/features/hosts/types/base';
  import { registry, serviceTypes } from '$lib/shared/stores/registry';
  import Tag from '$lib/shared/components/data/Tag.svelte';
  import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
  import { PortDisplay } from '$lib/shared/components/forms/selection/display/PortDisplay.svelte';
	import { InterfaceDisplay } from '$lib/shared/components/forms/selection/display/InterfaceDisplay.svelte';
  
  export let form: any;
  export let service: Service | null = null;
  export let open_ports: Port[] = [];
  export let host_interfaces: Interface[] = [];
  export let onChange: (updatedService: Service) => void = () => {};
  
  let serviceNameField: any;
  
  // Get service metadata from registry - use serviceTypes helper for consistency
  $: serviceMetadata = service ? serviceTypes.getItem(service.service_type.type) : null;
  
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
      `service_name_${service.service_type.type}`,
      service.name,
      [serviceNameValidator()]
    );
        
    // Register with parent form
    if (form) {
      form[`service_name_${service.service_type.type}`] = serviceNameField;
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

  // Get available ports (not already used by this service)
  $: availablePorts = open_ports.filter(port => 
    !service?.ports.some(servicePort => 
      servicePort.number === port.number && servicePort.protocol === port.protocol
    )
  );

  function handleAddPort(portId: string) {
    if (!service) return;
    
    // Parse the port ID back to find the port
    const [numberStr, protocol] = portId.split('-');
    const number = parseInt(numberStr);
    
    const selectedPort = open_ports.find(p => 
      p.number === number && p.protocol === protocol
    );
    
    if (!selectedPort) return;
    
    const updatedService = {
      ...service,
      ports: [...service.ports, selectedPort]
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
  
  // Interface binding management for the new architecture
  function handleAddInterface(interfaceId: string) {
    if (!service) return;
    
    // Don't add duplicate interfaces
    if (service.interface_bindings.includes(interfaceId)) return;
    
    const updatedService = {
      ...service,
      interface_bindings: [...service.interface_bindings, interfaceId]
    };
    
    onChange(updatedService);
  }
  
  function handleRemoveInterface(index: number) {
    if (!service) return;
    
    const updatedService = {
      ...service,
      interface_bindings: service.interface_bindings.filter((_, i) => i !== index)
    };
    
    onChange(updatedService);
  }
  
  // Get bound interfaces for display
  $: boundInterfaces = service ? 
    service.interface_bindings.map(id => host_interfaces.find(iface => iface.id === id)).filter(Boolean) as Interface[] : 
    [];
  
  // Available interfaces (not already bound)
  $: availableInterfaces = host_interfaces.filter(iface => 
    !service?.interface_bindings.includes(iface.id)
  );
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
    
    <!-- Interface Bindings -->
    {#if host_interfaces.length > 0}
      <div class="space-y-4">
        <ListManager
          label="Interface Bindings"
          helpText="Select which network interfaces this service is bound to"
          placeholder="Select an interface to bind..."
          emptyMessage="No interfaces bound to this service. Add one to get started."
          allowReorder={false}
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
      </div>
    {:else}
      <div class="p-4 bg-gray-800/50 rounded-lg">
        <div class="flex items-center gap-2 text-yellow-400 mb-2">
          <AlertTriangle size={16} />
          <span class="text-sm font-medium">No interfaces available</span>
        </div>
        <p class="text-xs text-gray-400">
          Add network interfaces to the host first, then bind services to specific interfaces.
        </p>
      </div>
    {/if}
    
    <!-- Ports Configuration using ListManager -->
    <div class="space-y-4">
      <ListManager
        label="Ports"
        helpText="Configure which ports this service uses"
        placeholder="Select a port to add"
        allowDuplicates={false}
        allowItemEdit={() => false}
        allowItemRemove={() => true}
        allowReorder={false}
        
        options={availablePorts}
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