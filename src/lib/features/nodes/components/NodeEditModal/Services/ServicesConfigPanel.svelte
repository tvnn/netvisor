<script lang="ts">
  import { field } from 'svelte-forms';
  import { required } from 'svelte-forms/validators';
  import { AlertCircle, Plus, Trash2 } from 'lucide-svelte';
  import type { Service, Port, Endpoint } from '$lib/features/services/types/base';
  import { getServiceDisplayName, formatServicePorts } from '$lib/features/services/types/base';
  import { registry } from '$lib/shared/stores/registry';
	import Tag from '$lib/shared/components/data/Tag.svelte';
  
  export let form: any;
  export let service: Service | null = null;
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
    
    confirmedField = field(
      `service_confirmed_${service.type}`,
      service.confirmed,
      []
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
      confirmed: $confirmedField.value
    };
    
    // Only trigger onChange if values actually changed
    if (updatedService.name !== service.name || updatedService.confirmed !== service.confirmed) {
      onChange(updatedService);
    }
  }
  
  // Port management functions
  function addPort() {
    if (!service) return;
    
    const newPort: Port = {
      number: 80,
      tcp: true,
      udp: false
    };
    
    const updatedService = {
      ...service,
      ports: [...service.ports, newPort]
    };
    
    onChange(updatedService);
  }
  
  function updatePort(index: number, updates: Partial<Port>) {
    if (!service) return;
    
    const updatedPorts = [...service.ports];
    updatedPorts[index] = { ...updatedPorts[index], ...updates };
    
    const updatedService = {
      ...service,
      ports: updatedPorts
    };
    
    onChange(updatedService);
  }
  
  function removePort(index: number) {
    if (!service) return;
    
    const updatedService = {
      ...service,
      ports: service.ports.filter((_, i) => i !== index)
    };
    
    onChange(updatedService);
  }
  
  // Event handlers for port fields
  function handlePortNumberChange(index: number, event: Event) {
    const target = event.target as HTMLInputElement;
    const portNumber = parseInt(target.value) || 80;
    updatePort(index, { number: portNumber });
  }
  
  function handleTcpChange(index: number, event: Event) {
    const target = event.target as HTMLInputElement;
    updatePort(index, { tcp: target.checked });
  }
  
  function handleUdpChange(index: number, event: Event) {
    const target = event.target as HTMLInputElement;
    updatePort(index, { udp: target.checked });
  }
  
  // Endpoint management functions
  function addEndpoint() {
    if (!service) return;
    
    const newEndpoint: Endpoint = {
      path: "/"
    };
    
    const updatedService = {
      ...service,
      endpoints: [...service.endpoints, newEndpoint]
    };
    
    onChange(updatedService);
  }
  
  function updateEndpoint(index: number, updates: Partial<Endpoint>) {
    if (!service) return;
    
    const updatedEndpoints = [...service.endpoints];
    updatedEndpoints[index] = { ...updatedEndpoints[index], ...updates };
    
    const updatedService = {
      ...service,
      endpoints: updatedEndpoints
    };
    
    onChange(updatedService);
  }
  
  function removeEndpoint(index: number) {
    if (!service) return;
    
    const updatedService = {
      ...service,
      endpoints: service.endpoints.filter((_, i) => i !== index)
    };
    
    onChange(updatedService);
  }
  
  // Event handler for endpoint path changes
  function handleEndpointPathChange(index: number, event: Event) {
    const target = event.target as HTMLInputElement;
    updateEndpoint(index, { path: target.value });
  }
</script>

{#if service && serviceMetadata}
  <div class="space-y-6">
    <!-- Service Info Header -->
    <div class="border-b border-gray-600 pb-4">
      <div class="flex gap-3 pb-1">
        <h3 class="text-lg font-medium text-white">{serviceMetadata.display_name}</h3>
        <Tag
          label={serviceMetadata.category}
          color={serviceMetadata.color}/>
      </div>
      <p class="text-sm text-gray-400">{serviceMetadata.description}</p>
      <div class="flex items-center gap-2 mt-2">
      </div>
    </div>
    
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
      
      <!-- Confirmed Status -->
      {#if confirmedField}
        <div class="space-y-2">
          <label class="flex items-center gap-3">
            <input
              type="checkbox"
              bind:checked={$confirmedField.value}
              class="w-4 h-4 text-blue-600 bg-gray-700 border-gray-600 rounded focus:ring-blue-500"
            />
            <span class="text-sm font-medium text-gray-300">Service Confirmed</span>
          </label>
          <p class="text-xs text-gray-400 ml-7">
            Mark as confirmed if you've verified this service is actually running
          </p>
        </div>
      {/if}
    </div>
    
    <!-- Ports Configuration -->
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <h4 class="text-sm font-medium text-gray-300">Ports</h4>
        <button
          type="button"
          on:click={addPort}
          class="flex items-center gap-2 px-3 py-1 text-xs bg-blue-600 text-white rounded hover:bg-blue-700"
        >
          <Plus size={14} />
          Add Port
        </button>
      </div>
      
      {#if service.ports.length === 0}
        <div class="text-center py-4 text-gray-400 text-sm">
          No ports configured. Click "Add Port" to add one.
        </div>
      {:else}
        <div class="space-y-3">
          {#each service.ports as port, index}
            <div class="flex items-center gap-3 p-3 bg-gray-700/30 rounded-lg">
              <div class="flex-1 grid grid-cols-3 gap-3">
                <div>
                  <div class="block text-xs font-medium text-gray-400 mb-1">Port Number</div>
                  <input
                    type="number"
                    min="1"
                    max="65535"
                    value={port.number}
                    on:input={(e) => handlePortNumberChange(index, e)}
                    class="w-full px-2 py-1 text-sm bg-gray-700 border border-gray-600 rounded text-white focus:ring-2 focus:ring-blue-500"
                  />
                </div>
                <div>
                  <div class="block text-xs font-medium text-gray-400 mb-1">Protocol</div>
                  <div class="flex gap-2">
                    <label class="flex items-center gap-1">
                      <input
                        type="checkbox"
                        checked={port.tcp}
                        on:change={(e) => handleTcpChange(index, e)}
                        class="w-3 h-3 text-blue-600 bg-gray-700 border-gray-600 rounded"
                      />
                      <span class="text-xs text-gray-300">TCP</span>
                    </label>
                    <label class="flex items-center gap-1">
                      <input
                        type="checkbox"
                        checked={port.udp}
                        on:change={(e) => handleUdpChange(index, e)}
                        class="w-3 h-3 text-blue-600 bg-gray-700 border-gray-600 rounded"
                      />
                      <span class="text-xs text-gray-300">UDP</span>
                    </label>
                  </div>
                </div>
                <div class="flex items-end">
                  <button
                    type="button"
                    on:click={() => removePort(index)}
                    class="flex items-center gap-1 px-2 py-1 text-xs bg-red-600 text-white rounded hover:bg-red-700"
                  >
                    <Trash2 size={12} />
                    Remove
                  </button>
                </div>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
    
    <!-- Endpoints Configuration -->
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <h4 class="text-sm font-medium text-gray-300">Endpoints</h4>
        <button
          type="button"
          on:click={addEndpoint}
          class="flex items-center gap-2 px-3 py-1 text-xs bg-blue-600 text-white rounded hover:bg-blue-700"
        >
          <Plus size={14} />
          Add Endpoint
        </button>
      </div>
      
      {#if service.endpoints.length === 0}
        <div class="text-center py-4 text-gray-400 text-sm">
          No endpoints configured. Click "Add Endpoint" to add one.
        </div>
      {:else}
        <div class="space-y-3">
          {#each service.endpoints as endpoint, index}
            <div class="flex items-center gap-3 p-3 bg-gray-700/30 rounded-lg">
              <div class="flex-1 grid grid-cols-2 gap-3">
                <div>
                  <div class="block text-xs font-medium text-gray-400 mb-1">Path</div>
                  <input
                    type="text"
                    value={endpoint.path || ""}
                    on:input={(e) => handleEndpointPathChange(index, e)}
                    placeholder="/api/health"
                    class="w-full px-2 py-1 text-sm bg-gray-700 border border-gray-600 rounded text-white focus:ring-2 focus:ring-blue-500"
                  />
                </div>
                <div class="flex items-end">
                  <button
                    type="button"
                    on:click={() => removeEndpoint(index)}
                    class="flex items-center gap-1 px-2 py-1 text-xs bg-red-600 text-white rounded hover:bg-red-700"
                  >
                    <Trash2 size={12} />
                    Remove
                  </button>
                </div>
              </div>
            </div>
          {/each}
        </div>
      {/if}
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