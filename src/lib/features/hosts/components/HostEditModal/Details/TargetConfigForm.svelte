<script lang="ts">
  import { AlertCircle, Globe, Server, TargetIcon } from 'lucide-svelte';
  import { field } from 'svelte-forms';
  import { required } from 'svelte-forms/validators';
  import type { HostTarget } from "$lib/features/hosts/types/targets";
  import { ipAddress, hostname } from '$lib/shared/components/forms/validators';
  import type { Host } from '$lib/features/hosts/types/base';

  export let form: any;
  export let formData: Host;
  export let target: HostTarget;

  // Create form fields based on target type
  let ipField: any;
  let hostnameField: any;

const targetTypes = [
  {
    value: 'IpAddress',
    label: 'IP Address',
    description: 'Connect directly to an IP address',
    icon: Server
  },
  {
    value: 'Hostname',
    label: 'Hostname/Domain',
    description: 'Connect using a hostname or domain name', 
    icon: Globe
  }
];

$: if (!formData.target) {
  formData.target = {
    type: 'IpAddress',
    config: { ip: '' }
  };
}

// Handle target type changes
function handleTargetTypeChange(event: Event) {
  const target = event.target as HTMLSelectElement;
  const newType = target.value as 'IpAddress' | 'Hostname';
  
  // Reset target config when type changes
  if (newType === 'IpAddress') {
    formData.target = {
      type: 'IpAddress',
      config: { ip: '' }
    };
  } else if (newType === 'Hostname') {
    formData.target = {
      type: 'Hostname', 
      config: { hostname: '' }
    };
  }
}

  // Initialize fields based on target type
  $: if (target.type === 'IpAddress') {
    if (!ipField) {
      let ipField = field('target_ip', target.config.ip || '', [required(), ipAddress()]);
      // Add to form if not already added
      if (form && !form.fields?.target_ip) {
        form.target_ip = ipField;
      }
    }
    // Update field value when target changes
    if (ipField && $ipField.value !== (target.config.ip || '')) {
      ipField.set(target.config.ip || '');
    }
  }

  $: if (target.type === 'Hostname') {
    if (!hostnameField) {
      hostnameField = field('target_hostname', target.config.hostname || '', [required(), hostname()]);
      // Add to form if not already added
      if (form && !form.fields?.target_hostname) {
        form.target_hostname = hostnameField;
      }
    }
    // Update field value when target changes
    if (hostnameField && $hostnameField.value !== (target.config.hostname || '')) {
      hostnameField.set(target.config.hostname || '');
    }
  }

  // Update target config when form fields change
  $: if (target.type === 'IpAddress' && ipField && $ipField) {
    target.config.ip = $ipField.value;
  }

  $: if (target.type === 'Hostname' && hostnameField && $hostnameField) {
    target.config.hostname = $hostnameField.value;
  }
</script>

<h4 class="text-md font-medium text-white mb-4 flex items-center gap-2">
  <TargetIcon class="w-5 h-5" />
  Connection Target
</h4>
<div class="flex gap-6 items-start">
<div class="flex flex-col space-y-2 w-1/5">
    <label for="target_type" class="block text-sm font-medium text-gray-300">
      Target Type
      <span class="text-red-400 ml-1">*</span>
    </label>
    <select
      id="target_type"
      value={formData.target?.type || 'IpAddress'}
      on:change={handleTargetTypeChange}
      class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white 
              focus:outline-none focus:ring-2 focus:ring-blue-500"
    >
      {#each targetTypes as targetType}
        <option value={targetType.value}>{targetType.label}</option>
      {/each}
    </select>
    <p class="text-xs text-gray-400">
      How should NetVisor connect to this host?
    </p>
  </div>
<div class="flex flex-col flex-grow">
{#if formData.target}
<div class="space-y-4">
  {#if target.type === 'IpAddress'}
    <!-- IP Address Configuration -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div class="md:col-span-2 space-y-2">
        <label for="ip_address" class="block text-sm font-medium text-gray-300">
          IP Address
          <span class="text-red-400 ml-1">*</span>
        </label>
        <input
          id="ip_address"
          type="text"
          on:input={(e) => {
            const target = e.target as HTMLInputElement;
            if (ipField && target) ipField.set(target.value);
          }}
          class="w-full px-3 py-2 bg-gray-700 border rounded-md text-white 
                 focus:outline-none focus:ring-2 focus:ring-blue-500
                 {ipField && $ipField.errors.length > 0 ? 'border-red-500' : 'border-gray-600'}"
          placeholder="192.168.1.100"
        />
        {#if ipField && $ipField.errors.length > 0}
          <div class="flex items-center gap-2 text-red-400">
            <AlertCircle size={16} />
            <p class="text-xs">{$ipField.errors[0]}</p>
          </div>
        {/if}
        <p class="text-xs text-gray-400">
          The IP address where this host can be reached
        </p>
      </div>
    </div>
    
  {:else if target.type === 'Hostname'}
    <!-- Hostname Configuration -->
    <div class="space-y-4">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">        
        <div class="space-y-2">
          <label for="service_hostname" class="block text-sm font-medium text-gray-300">
            Hostname
            <span class="text-red-400 ml-1">*</span>
          </label>
          <input
            id="service_hostname"
            type="text"
            on:input={(e) => {
              const target = e.target as HTMLInputElement;
              if (hostnameField && target) hostnameField.set(target.value);
            }}
            class="w-full px-3 py-2 bg-gray-700 border rounded-md text-white 
                   focus:outline-none focus:ring-2 focus:ring-blue-500
                   {hostnameField && $hostnameField.errors.length > 0 ? 'border-red-500' : 'border-gray-600'}"
            placeholder="api.example.com"
          />
          {#if hostnameField && $hostnameField.errors.length > 0}
            <div class="flex items-center gap-2 text-red-400">
              <AlertCircle size={16} />
              <p class="text-xs">{$hostnameField.errors[0]}</p>
            </div>
          {/if}
          <p class="text-xs text-gray-400">
            The hostname or domain name for this host
          </p>
        </div>
      </div>
    </div>
  {/if}
</div>
{/if}
</div>
</div>