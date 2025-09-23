<!-- src/lib/features/hosts/components/HostEditModal/Services/ServicesConfigPanel.svelte -->
<script lang="ts">
  import { field } from 'svelte-forms';
  import { AlertCircle, AlertTriangle } from 'lucide-svelte';
  import type { Service, Port } from '$lib/features/services/types/base';
  import type { Interface } from '$lib/features/hosts/types/base';
  import { entities, metadata, serviceDefinitions } from '$lib/shared/stores/metadata';
  import Tag from '$lib/shared/components/data/Tag.svelte';
  import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
  import { PortDisplay } from '$lib/shared/components/forms/selection/display/PortDisplay.svelte';
	import { InterfaceDisplay } from '$lib/shared/components/forms/selection/display/InterfaceDisplay.svelte';
	import InlineWarning from '$lib/shared/components/feedback/InlineWarning.svelte';
	import type { FormApi, FormType } from '$lib/shared/components/forms/types';
	import { required } from 'svelte-forms/validators';
	import { onMount } from 'svelte';
	import { pushWarning } from '$lib/shared/stores/feedback';
	import TextInput from '$lib/shared/components/forms/input/TextInput.svelte';
	import { maxLength } from '$lib/shared/components/forms/validators';
	import ConfigHeader from '$lib/shared/components/forms/config/ConfigHeader.svelte';
  
  export let formApi: FormApi;
  export let service: Service;
  export let host_interfaces: Interface[] = [];
  export let onChange: (updatedService: Service) => void = () => {};
  
  let currentServiceId: string = service.id;

  const getNameField = () => {
    return field(
      `service_name_${currentServiceId}`,
      service.name,
      [required(), maxLength(100)]
    );
  }

  let nameField = getNameField();

  $: serviceMetadata = service ? serviceDefinitions.getItem(service.service_definition) : null;
  
  $: if (service.id !== currentServiceId) {
    currentServiceId = service.id
    nameField = getNameField();
  }    

  // Update service when field values change
  $: if ($nameField) {
    const updatedService: Service = {
      ...service,
      name: $nameField.value,
    };
    
    // Only trigger onChange if values actually changed
    if (updatedService.name !== service.name) {
      onChange(updatedService);
    }
  }

  function handleCreateNewPort() {
    if (!service) return;
    
    // Create a new port with default values in editing state
    const newPort: Port = {
      number: 80,  // Default port number
      protocol: 'Tcp'  // Default protocol
    };
    
    const updatedService = {
      ...service,
      ports: [...service.ports, newPort]
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

  function handleUpdatePort(port: Port, index: number) {
    if (!service) return;

    const updatedPorts = [...service.ports];
    updatedPorts[index] = port;
    
    const updatedService = {
      ...service,
      ports: updatedPorts  // Use the new array
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
          pushWarning(`⚠️ Interface binding ${id} not found in host interfaces for service ${service.name}`);
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

</script>

{#if service && serviceMetadata}
  <div class="space-y-6">
    
    <ConfigHeader title={serviceMetadata.name} subtitle={serviceMetadata.description} />

    <!-- Basic Configuration -->
    <div class="space-y-4">
      <!-- Service Name Field -->
      {#if nameField}
        <TextInput 
            label="Name" 
            id="service_name_{service.id}" 
            {formApi}
            required={true}
            placeholder="Enter a descriptive name..."
            field={nameField}/>
      {/if}
    </div>
    
    <!-- Interface Bindings -->
    <div class="space-y-4">
      {#key service.id}
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
      {/key}
    </div>
    
    <!-- Ports Configuration using ListManager -->
    <div class="space-y-4">
      <ListManager
        label="Ports"
        helpText="Configure which ports this service uses"
        placeholder="Select a port to add"
        createNewLabel="New Port"
        allowDuplicates={false}
        allowItemEdit={() => true}
        allowItemRemove={() => true}
        allowReorder={false}
        allowCreateNew={true}
        
        options={[]}
        items={service.ports}
        
        optionDisplayComponent={PortDisplay}
        itemDisplayComponent={PortDisplay}
        
        onCreateNew={handleCreateNewPort}
        onRemove={handleRemovePort}
        onEdit={handleUpdatePort}
      />
    </div>
  </div>
{/if}