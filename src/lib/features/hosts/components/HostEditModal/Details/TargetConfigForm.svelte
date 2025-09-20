<script lang="ts">
  import { AlertCircle, Globe, Network, TargetIcon } from 'lucide-svelte';
  import type { Host, HostTarget } from '$lib/features/hosts/types/base';
  import RichSelect from '$lib/shared/components/forms/selection/RichSelect.svelte';
  import { InterfaceDisplay } from '$lib/shared/components/forms/selection/display/InterfaceDisplay.svelte';
	import { field } from 'svelte-forms';
  import { ipAddress } from '$lib/shared/components/forms/validators';

  export let form: any;
  export let formData: Host;
  export let target: HostTarget;

  // Form fields
  let selectedInterfaceId: string = '';
  let ipAddressField: any;

  const targetTypes = [
    {
      value: 'Interface',
      label: formData.interfaces.length != 0 ? 'Network Interface' : 'No Network Interfaces Configured',
      description: 'Connect using an IP from a network interface',
      disabled: formData.interfaces.length === 0,
      icon: Network
    },
    {
      value: 'Hostname',
      label: formData.hostname.length > 0 ? 'Hostname' : "No Hostname Configured",
      description: 'Connect using the host\'s hostname field', 
      disabled: formData.hostname.length == 0,
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
    const newType = targetElement.value as 'Interface' | 'Hostname';
    
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

  // Update selected interface when target changes
  $: if (formData.target?.type === 'Interface') {
    selectedInterfaceId = formData.target.config || '';
  } else if (formData.target?.type === 'ExternalIp') {
    ipAddressField = field(
      `target_ip`,
      formData.target.config || '',
      [ipAddress()]
    );
    form[`target_ip`] = ipAddressField;
  }
</script>

<h4 class="text-md font-medium text-white mb-4 flex items-center gap-2">
  <TargetIcon class="w-5 h-5" />
  Connection Target
</h4>

<div class="flex gap-6 items-start">
  <!-- Target Type Selection -->
  <div class="flex flex-col space-y-2 w-1/3">
    <label for="target_type" class="block text-sm font-medium text-gray-300">
      Target Type
      <span class="text-red-400 ml-1">*</span>
    </label>
    <select
      id="target_type"
      value={formData.target?.type || 'Interface'}
      on:change={handleTargetTypeChange}
      class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white 
              focus:outline-none focus:ring-2 focus:ring-blue-500"
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
                selectedValue={selectedInterfaceId}
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
        {:else if formData.target.type === 'ExternalIp'}
          <!-- Hostname Display -->
          <div class="space-y-2">
                <label for="ip_address" class="block text-sm font-medium text-gray-300">
                  External IP Address <span class="text-red-400">*</span>
                </label>
                <input
                  id="ip_address"
                  type="text"
                  bind:value={formData.target.config}
                  class="w-full px-3 py-2 bg-gray-700 border rounded-md text-white 
                        focus:outline-none focus:ring-2 focus:ring-blue-500
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