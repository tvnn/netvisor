<script lang="ts">
  import { field } from 'svelte-forms';
  import { AlertCircle } from 'lucide-svelte';
	import type { Interface } from '$lib/features/hosts/types/base';
	import { ipAddress, mac } from '$lib/shared/components/forms/validators';
	import { entities } from '$lib/shared/stores/metadata';
	import { required } from 'svelte-forms/validators';
  
  export let form: any;
  export let iface: Interface | null = null;
  export let subnet: Subnet | null = null;
  export let onChange: (updatedIface: Interface) => void = () => {};
  
  let ipAddressField: any;
  let macAddressField: any;
  
  // Initialize form fields when membership changes
  $: if (iface && subnet) {
    ipAddressField = field(
      `interface_ip_${iface.subnet_id}`,
      iface.ip_address,
      [required(), ipAddress()]
    );
    
    macAddressField = field(
      `interface_mac_${iface.subnet_id}`,
      iface.mac_address || '',
      [mac()]
    );
    
    // Register with parent form
    if (form) {
      form[`interface_ip_${iface.subnet_id}`] = ipAddressField;
      form[`interface_mac_${iface.subnet_id}`] = macAddressField;
    }
  }
  
  // Update interface when field values change
  $: if (iface && ipAddressField && macAddressField && $ipAddressField && $macAddressField) {
    const updatedIface: Interface = {
      ...iface,
      ip_address: $ipAddressField.value,
      mac_address: $macAddressField.value || undefined
    };
    
    // Only trigger onChange if values actually changed
    if (updatedIface.ip_address !== iface.ip_address || 
        updatedIface.mac_address !== iface.mac_address) {
      onChange(updatedIface);
    }
  }

  let colorHelper = entities.getColorHelper("Host")
</script>

{#if iface && subnet}
  <div class="space-y-6">
    <!-- Subnet Info Header -->
    <div class="border-b border-gray-600 pb-4">
      <h3 class="text-lg font-medium text-white">{subnet.name}</h3>
      <p class="text-sm text-gray-400">{subnet.cidr}</p>
      {#if subnet.description}
        <p class="text-xs text-gray-500 mt-1">{subnet.description}</p>
      {/if}
    </div>
    
    <!-- Network Configuration -->
    <div class="space-y-4">
      <h4 class="text-sm font-medium text-gray-300">Network Configuration</h4>
      
      <!-- IP Address Field -->
      {#if ipAddressField}
        <div class="space-y-2">
          <label for="ip_address" class="block text-sm font-medium text-gray-300">
            IP Address <span class="text-red-400">*</span>
          </label>
          <input
            id="ip_address"
            type="text"
            bind:value={$ipAddressField.value}
            class="w-full px-3 py-2 bg-gray-700 border rounded-md text-white 
                   focus:outline-none focus:ring-2
                   {$ipAddressField.errors.length > 0 ? 'border-red-500' : 'border-gray-600'}"
            placeholder="192.168.1.100"
          />
          {#if $ipAddressField.errors.length > 0}
            <div class="flex items-center gap-2 text-red-400">
              <AlertCircle size={16} />
              <p class="text-xs">{$ipAddressField.errors[0]}</p>
            </div>
          {/if}
          <p class="text-xs text-gray-400">
            Must be within {subnet.cidr}
          </p>
        </div>
      {/if}
      
      <!-- MAC Address Field -->
      {#if macAddressField}
        <div class="space-y-2">
          <label for="mac_address" class="block text-sm font-medium text-gray-300">
            MAC Address <span class="text-gray-500">(optional)</span>
          </label>
          <input
            id="mac_address"
            type="text"
            bind:value={$macAddressField.value}
            class="w-full px-3 py-2 bg-gray-700 border rounded-md text-white 
                   focus:outline-none focus:ring-2
                   {$macAddressField.errors.length > 0 ? 'border-red-500' : 'border-gray-600'}"
            placeholder="00:1B:44:11:3A:B7"
          />
          {#if $macAddressField.errors.length > 0}
            <div class="flex items-center gap-2 text-red-400">
              <AlertCircle size={16} />
              <p class="text-xs">{$macAddressField.errors[0]}</p>
            </div>
          {/if}
          <p class="text-xs text-gray-400">
            Format: XX:XX:XX:XX:XX:XX or XX-XX-XX-XX-XX-XX
          </p>
        </div>
      {/if}
    </div>
    
    <!-- Additional Info -->
    <div class="bg-gray-700/30 rounded-lg p-4">
      <h5 class="text-sm font-medium text-gray-300 mb-2">Network Details</h5>
      <div class="space-y-1 text-xs text-gray-400">
        <div>Subnet: {subnet.cidr}</div>
        {#if subnet.dns_resolvers?.length > 0}
          <div>DNS Servers: {subnet.dns_resolvers.length} configured</div>
        {/if}
        {#if subnet.gateways?.length > 0}
          <div>Gateways: {subnet.gateways.length} configured</div>
        {/if}
      </div>
    </div>
  </div>
{:else}
  <div class="flex-1 min-h-0 flex items-center justify-center text-gray-400">
    <div class="text-center">
      <div class="text-lg mb-2">No subnet selected</div>
      <div class="text-sm">Select a subnet from the list to configure it</div>
    </div>
  </div>
{/if}