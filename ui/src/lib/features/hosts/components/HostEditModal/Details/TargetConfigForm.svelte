<script lang="ts">
  import { AlertCircle, Globe, Network, TargetIcon } from 'lucide-svelte';
  import type { Host, HostTarget, ServiceBinding } from '$lib/features/hosts/types/base';
  import RichSelect from '$lib/shared/components/forms/selection/RichSelect.svelte';
  import { InterfaceDisplay } from '$lib/shared/components/forms/selection/display/InterfaceDisplay.svelte';
	import { field } from 'svelte-forms';
  import { ipAddress } from '$lib/shared/components/forms/validators';
	import { required } from 'svelte-forms/validators';
	import { onMount } from 'svelte';
	import type { FieldType, FormApi, FormType } from '$lib/shared/components/forms/types';
	import InlineWarning from '$lib/shared/components/feedback/InlineWarning.svelte';
	import { getServicesForHost } from '$lib/features/services/store';
	import { getInterfaceFromId, getPortFromId, serviceBindingIdToObj, serviceBindingToId } from '$lib/features/hosts/store';
	import { uuidv4Sentinel } from '$lib/shared/utils/formatting';
	import { ServiceBindingDisplay } from '$lib/shared/components/forms/selection/display/ServiceBindingDisplay.svelte';

  export let formApi: FormApi;
  export let form: FormType;
  export let formData: Host;

  let currentTargetType = formData.target.type
  let selectedBinding: ServiceBinding;

  if (formData.target.type == 'ServiceBinding') {
    selectedBinding = formData.target.config
  }

  $: serviceBindings = getServicesForHost(formData.id).flatMap(s => s.interface_bindings
    .flatMap(interface_id => s.port_bindings
      .map(port_id => getPortFromId(port_id))
      .filter(port => port != undefined)
      .filter(port => port.protocol == 'Tcp')
      .map(port => {
        return {
          service_id: s.id,
          interface_id,
          port_id: port.id
        }
      })
    )
  )

  $: hostnameField = form.getField('hostname');
  $: has_hostname = hostnameField ? $hostnameField.value.length > 0 : false;

  // Form fields
  $: targetTypes = [
    {
      value: 'ServiceBinding',
      label: formData.interfaces.length != 0 ? 'Service Binding' : 'No Services Configured',
      description: 'Connect to a service port on a network interface',
      disabled: serviceBindings.length === 0,
      icon: Network
    },
    {
      value: 'Hostname',
      label: has_hostname ? 'Hostname' : "No Hostname Configured",
      description: 'Connect using the host\'s hostname field', 
      disabled: !has_hostname,
      icon: Globe
    },
    {
      value: 'None',
      label: 'None',
      description: 'No connection set for this host', 
      disabled: false,
      icon: Globe
    }
  ];

  // Initialize target if not set
  $: if (!formData.target) {
    formData.target = {
      type: 'Hostname'
    };
  }

  // Handle target type changes
  function handleTargetTypeChange(event: Event) {
    const targetElement = event.target as HTMLSelectElement;
    const newType = targetElement.value;
    
    // Reset target config when type changes
    if (newType === 'ServiceBinding') {
      let binding = serviceBindings[0]
      formData.target = {
        type: 'ServiceBinding',
        config: binding
      };
      selectedBinding = {service_id: uuidv4Sentinel, interface_id: uuidv4Sentinel, port_id: uuidv4Sentinel};
    } else if (newType === 'Hostname') {
      formData.target = {
        type: 'Hostname'
      };
    } else if (newType === 'None') {
      formData.target = {
        type: 'None'
      };
    } 
    
    // Force reactivity update
    formData = { ...formData };
  }

  // Handle interface selection
  function handleServiceBindingSelect(binding: string) {
    let parsed_binding = serviceBindingIdToObj(binding)
    if (parsed_binding) {
      selectedBinding = parsed_binding;
      if (formData.target.type == 'ServiceBinding') {
        formData.target.config = parsed_binding;
      }
    }
  }

</script>

<div class="flex gap-6 items-start">
  <!-- Target Type Selection -->
  <div class="flex flex-col space-y-2 w-1/3">
    <label for="target_type" class="block text-sm font-medium text-gray-300">
      Link Type
    </label>
    <select
      id="target_type"
      value={formData.target?.type || 'Interface'}
      on:change={handleTargetTypeChange}
      class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white 
              focus:outline-none focus:ring-2"
    >
      {#each targetTypes as targetType}
        <option disabled={targetType.disabled} value={targetType.value}>{targetType.label}</option>
      {/each}
    </select>
    <p class="text-xs text-gray-400">
      How should NetVisor display a link for this host?
    </p>
  </div>

  <!-- Target Configuration -->
  <div class="flex flex-col flex-grow">
    {#if formData.target}
      <div class="space-y-4">
        {#if formData.target.type === 'ServiceBinding'}
          <!-- Interface Selection -->
          <div class="space-y-2">
            <label for="interface_select" class="block text-sm font-medium text-gray-300">
              Service Binding
              <span class="text-red-400 ml-1">*</span>
            </label>

            {#if formData.interfaces.length == 0} 
              <InlineWarning title="No services available" body="No services available. Add a service or change target type."/>
            {:else} 
              <RichSelect
                selectedValue={serviceBindingToId(selectedBinding)}
                options={serviceBindings}
                placeholder="Select a service binding..."
                displayComponent={ServiceBindingDisplay}
                onSelect={handleServiceBindingSelect}
              />
            {/if}
          </div>
          
        {:else if formData.target.type === 'Hostname'}
          <!-- Hostname Display -->
          <div class="space-y-2">
            <div class="block text-sm font-medium text-gray-300">
              Hostname
            </div>
              <div class="w-full px-3 py-2 bg-gray-800/50 border border-gray-500 rounded-md text-gray-200 flex items-center gap-2">
                <span class="font-mono">{formData.hostname}</span>
              </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>