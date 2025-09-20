<script lang="ts">
  import { onMount } from 'svelte';
  import { AlertCircle } from 'lucide-svelte';
  import { field } from 'svelte-forms';
  import { required } from 'svelte-forms/validators';
  import type { Host } from '$lib/features/hosts/types/base';
  import { maxLength } from '$lib/shared/components/forms/validators';
  import TargetConfigForm from './TargetConfigForm.svelte';
  
  export let form: any;
  export let formData: Host;
  
  // Create form fields with validation
  const name = field('name', formData.name, [required(), maxLength(100)]);
  const description = field('description', formData.description || '', [maxLength(500)]);
  const hostname = field('hostname', formData.hostname || '');
  
  // Add fields to form
  onMount(() => {
    form.name = name;
    form.description = description;
    form.hostname = hostname;
  });
  
  // Update formData when field values change
  $: formData.name = $name.value;
  $: formData.description = $description.value;
  $: formData.hostname = $hostname.value;
  
  // Initialize target if needed
  $: if (!formData.target) {
    formData.target = {
      type: 'Interface',
      config: ''
    };
  }
</script>

<div class="space-y-8">    
  <div class="grid grid-cols-2 gap-6">
    <!-- Name -->
    <div class="space-y-2">
      <label for="host_name" class="block text-sm font-medium text-gray-300">
        Name
        <span class="text-red-400 ml-1">*</span>
      </label>
      <input
        id="host_name"
        type="text"
        bind:value={$name.value}
        class="w-full px-3 py-2 bg-gray-700 border rounded-md text-white 
               focus:outline-none focus:ring-2 focus:ring-blue-500
               {$name.errors.length > 0 ? 'border-red-500' : 'border-gray-600'}"
        placeholder="Enter a descriptive name for this host..."
      />
      {#if $name.errors.length > 0}
        <div class="flex items-center gap-2 text-red-400">
          <AlertCircle size={16} />
          <p class="text-xs">{$name.errors[0]}</p>
        </div>
      {/if}
      <p class="text-xs text-gray-400">
        A meaningful name like "API Server", "Database", or "Load Balancer"
      </p>
    </div>

    <!-- Hostname -->
    <div class="space-y-2">
      <label for="host_domain" class="block text-sm font-medium text-gray-300">
        Hostname
      </label>
      <input
        id="host_domain"
        type="text"
        bind:value={$hostname.value}
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white 
               focus:outline-none focus:ring-2 focus:ring-blue-500"
        placeholder="api.example.com"
      />
      <p class="text-xs text-gray-400">
        The hostname or domain name for this host
      </p>
    </div>
  </div>
  
  <!-- Description -->
  <div class="space-y-2">
    <label for="host_description" class="block text-sm font-medium text-gray-300">
      Description
    </label>
    <textarea
      id="host_description"
      bind:value={$description.value}
      rows="3"
      class="w-full px-3 py-2 bg-gray-700 border rounded-md text-white 
              focus:outline-none focus:ring-2 focus:ring-blue-500 resize-vertical
              {$description.errors.length > 0 ? 'border-red-500' : 'border-gray-600'}"
      placeholder="Optional description of this host's role or purpose..."
    ></textarea>
    {#if $description.errors.length > 0}
      <div class="flex items-center gap-2 text-red-400">
        <AlertCircle size={16} />
        <p class="text-xs">{$description.errors[0]}</p>
      </div>
    {/if}
    <p class="text-xs text-gray-400">
      Describe what this host does or its role in your infrastructure
    </p>
  </div>

  <!-- Target Configuration -->
  <TargetConfigForm 
    {form}
    {formData}
    target={formData.target}
  />
</div>