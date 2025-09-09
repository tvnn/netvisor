<script lang="ts">
  import { field } from 'svelte-forms';
  import { AlertCircle } from 'lucide-svelte';
	import type { HostSubnetMembership } from '$lib/features/hosts/types/base';
  
  export let form: any;
  export let membership: HostSubnetMembership | null = null;
  export let subnet: Subnet | null = null;
  export let onChange: (updatedMembership: HostSubnetMembership) => void = () => {};
  
  let ipAddressField: any;
  let macAddressField: any;
  
    const ipValidator = () => (value: string) => {
    if (!value) return { name: 'required', valid: false };
    
    const ipRegex = /^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$/;
    if (!ipRegex.test(value)) {
        return { name: 'invalidIp', valid: false };
    }
    
    const octets = value.split('.').map(Number);
    if (octets.some(octet => octet < 0 || octet > 255)) {
        return { name: 'invalidRange', valid: false };
    }
    
    return { name: 'validIp', valid: true };
    };
    
    const macValidator = () => (value: any) => {
    if (!value) return { name: 'validMac', valid: true }; // Optional field
    
    const macRegex = /^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$/;
    if (!macRegex.test(value)) {
        return { name: 'invalidMac', valid: false };
    }
    
    return { name: 'validMac', valid: true };
    };
  
  // Initialize form fields when membership changes
  $: if (membership && subnet) {
    ipAddressField = field(
      `subnet_ip_${membership.subnet_id}`,
      membership.ip_address,
      [ipValidator()]
    );
    
    macAddressField = field(
      `subnet_mac_${membership.subnet_id}`,
      membership.mac_address || '',
      [macValidator()]
    );
    
    // Register with parent form
    if (form) {
      form[`subnet_ip_${membership.subnet_id}`] = ipAddressField;
      form[`subnet_mac_${membership.subnet_id}`] = macAddressField;
    }
  }
  
  // Update membership when field values change
  $: if (membership && ipAddressField && macAddressField && $ipAddressField && $macAddressField) {
    const updatedMembership: HostSubnetMembership = {
      ...membership,
      ip_address: $ipAddressField.value,
      mac_address: $macAddressField.value || undefined
    };
    
    // Only trigger onChange if values actually changed
    if (updatedMembership.ip_address !== membership.ip_address || 
        updatedMembership.mac_address !== membership.mac_address) {
      onChange(updatedMembership);
    }
  }
</script>

{#if membership && subnet}
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
                   focus:outline-none focus:ring-2 focus:ring-blue-500
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
                   focus:outline-none focus:ring-2 focus:ring-blue-500
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