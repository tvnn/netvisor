<!-- src/lib/features/nodes/components/NodeEditModal/Details/TargetConfigForm.svelte -->
<script lang="ts">
  import { AlertCircle } from 'lucide-svelte';
  import { field } from 'svelte-forms';
  import { required } from 'svelte-forms/validators';
  import type { NodeTarget } from "$lib/features/nodes/types/targets";
  import { ipAddress, hostname } from '$lib/shared/components/forms/validators';

  export let form: any;
  export let target: NodeTarget;

  // Create form fields based on target type
  let ipField: any;
  let hostnameField: any;

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
          The IP address where this node can be reached
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
            The hostname or domain name for this node
          </p>
        </div>
      </div>
    </div>
  {/if}
</div>