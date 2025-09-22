<script lang="ts">
  import { AlertCircle, Globe, Network, TargetIcon } from 'lucide-svelte';
  import type { Host, HostTarget } from '$lib/features/hosts/types/base';
  import RichSelect from '$lib/shared/components/forms/selection/RichSelect.svelte';
  import { InterfaceDisplay } from '$lib/shared/components/forms/selection/display/InterfaceDisplay.svelte';
	import { field } from 'svelte-forms';
  import { ipAddress } from '$lib/shared/components/forms/validators';
	import { required } from 'svelte-forms/validators';
	import { onMount } from 'svelte';
	import type { FieldType, FormApi, FormType } from '$lib/shared/components/forms/types';

  export let formApi: FormApi;
  export let form: FormType;
  export let formData: Host;

  let currentTargetType = formData.target.type
  let selectedInterfaceId: string = '';

  const getIpField = () => {
    return field(
      'target_ip',
      formData.target?.type === 'ExternalIp' ? formData.target.config : '',
      [ipAddress(), required()],
      {
        checkOnInit: false,
      }
    )
  }

  let ipAddressField = getIpField();

  $: hostnameField = form.getField('hostname');
  $: has_hostname = hostnameField ? $hostnameField.value.length > 0 : false;

  // Form fields
  $: targetTypes = [
    {
      value: 'Interface',
      label: formData.interfaces.length != 0 ? 'Network Interface' : 'No Network Interfaces Configured',
      description: 'Connect using an IP from a network interface',
      disabled: formData.interfaces.length === 0,
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
      value: 'ExternalIp',
      label: 'External IP',
      description: 'Connect using an external (non-local) IP. For local IPs, use the "Network Interface" option', 
      disabled: false,
      icon: Globe
    }
  ];

  // Update validation when target type changes
  $: if (formData.target.type != currentTargetType) {
    currentTargetType = formData.target.type
    if (formData.target.type === 'ExternalIp') {
      let ipField = getIpField()
      formApi.registerField('ip', ipField)
      ipAddressField = ipField;
    } else {
      formApi.unregisterField('ip')
    }
  }

  // Initialize target if not set
  $: if (!formData.target) {
    formData.target = {
      type: 'Interface',
      config: ''
    };
  }

  // Handle target type changes
  function handleTargetTypeChange(event: Event) {
    const targetElement = event.target as HTMLSelectElement;
    const newType = targetElement.value;
    
    // Reset target config when type changes
    if (newType === 'Interface') {
      formData.target = {
        type: 'Interface',
        config: ''
      };
      selectedInterfaceId = '';
    } else if (newType === 'Hostname') {
      formData.target = {
        type: 'Hostname'
      };
    } else if (newType === 'ExternalIp') {
      formData.target = {
        type: 'ExternalIp',
        config: ''
      };
    } 
    
    // Force reactivity update
    formData = { ...formData };
  }

  // Handle interface selection
  function handleInterfaceSelect(interfaceId: string) {
    selectedInterfaceId = interfaceId;
    if (formData.target?.type === 'Interface') {
      formData.target.config = interfaceId;
    }
  }

  // Track IP address changes in formData
  $: if (formData.target.type == 'ExternalIp') {
    formData.target.config = $ipAddressField?.value || ''
  }

</script>

<div class="flex gap-6 items-start">
  <!-- Target Type Selection -->
  <div class="flex flex-col space-y-2 w-1/3">
    <label for="target_type" class="block text-sm font-medium text-gray-300">
      Target Type
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
      How should NetVisor connect to this host?
    </p>
  </div>

  <!-- Target Configuration -->
  <div class="flex flex-col flex-grow">
    {#if formData.target}
      <div class="space-y-4">
        {#if formData.target.type === 'Interface'}
          <!-- Interface Selection -->
          <div class="space-y-2">
            <label for="interface_select" class="block text-sm font-medium text-gray-300">
              Network Interface
              <span class="text-red-400 ml-1">*</span>
            </label>
            
              <RichSelect
                selectedValue={selectedInterfaceId || formData.interfaces[0].id}
                options={formData.interfaces}
                placeholder="Select a network interface..."
                displayComponent={InterfaceDisplay}
                onSelect={handleInterfaceSelect}
              />
              
              <p class="text-xs text-gray-400">
                Choose which network interface to use for connecting to this host
              </p>
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
        {:else if formData.target.type === 'ExternalIp' && $ipAddressField}
          <!-- Hostname Display -->
          <div class="space-y-2">
                <label for="ip_address" class="block text-sm font-medium text-gray-300">
                  External IP Address <span class="text-red-400">*</span>
                </label>
                <input
                  id="target_ip"
                  type="text"
                  bind:value={$ipAddressField.value}
                  class="w-full px-3 py-2 bg-gray-700 border rounded-md text-white 
                        focus:outline-none focus:ring-2
                        {$ipAddressField.errors.length > 0 ? 'border-red-500' : 'border-gray-600'}"
                  placeholder="1.1.1.1"
                />
                {#if $ipAddressField.errors.length > 0}
                  <div class="flex items-center gap-2 text-red-400">
                    <AlertCircle size={16} />
                    <p class="text-xs">{$ipAddressField.errors[0]}</p>
                  </div>
                {/if}
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>