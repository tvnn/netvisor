<script lang="ts">
  import { onMount } from 'svelte';
  import { Globe, Server, FileText, Tag as TagIcon, AlertCircle, TargetIcon } from 'lucide-svelte';
  import { field } from 'svelte-forms';
  import { required } from 'svelte-forms/validators';
  import type { Node } from '$lib/features/nodes/types/base';
  import { maxLength, validNodeType } from '$lib/shared/components/forms/validators';
	import { nodeTargets } from '$lib/shared/stores/registry';
  
  export let form: any;
  export let formData: Node;
  
  // Create form fields with validation
  const name = field('name', formData.name, [required(), maxLength(100)]);
  const description = field('description', formData.description || '', [maxLength(500)]);
  const hostname = field('hostname', formData.hostname || '');
  
  // Add fields to form
  onMount(() => {
    form.name = name
    form.description = description
    form.hostname = hostname
  });
  
  // Update formData when field values change
  $: formData.name = $name.value;
  $: formData.description = $description.value;
  $: formData.hostname = $hostname.value;

  $: targetTypeOptions = nodeTargets.getItems().map(t => {return {value:t.id, label: t.display_name, description: t.description, icon: t.icon}});
  
  // Initialize target if needed
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

</script>

<div class="space-y-8">    
    <div class="grid grid-cols-2 gap-6">
      <!-- Node Name -->
      <div class="space-y-2">
        <label for="node_name" class="block text-sm font-medium text-gray-300">
          Name
          <span class="text-red-400 ml-1">*</span>
        </label>
        <input
          id="node_name"
          type="text"
          bind:value={$name.value}
          class="w-full px-3 py-2 bg-gray-700 border rounded-md text-white 
                 focus:outline-none focus:ring-2 focus:ring-blue-500
                 {$name.errors.length > 0 ? 'border-red-500' : 'border-gray-600'}"
          placeholder="Enter a descriptive name for this node..."
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
        <label for="node_domain" class="block text-sm font-medium text-gray-300">
          Hostname
        </label>
        <input
          id="node_domain"
          type="text"
          bind:value={$hostname.value}
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white 
                 focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="production, staging, development..."
        />
        <p class="text-xs text-gray-400">
          Environment or domain this node belongs to
        </p>
      </div>
    </div>
    
    <!-- Description -->
    <div class="space-y-2">
      <label for="node_description" class="block text-sm font-medium text-gray-300">
        Description
      </label>
      <textarea
        id="node_description"
        bind:value={$description.value}
        rows="3"
        class="w-full px-3 py-2 bg-gray-700 border rounded-md text-white 
                focus:outline-none focus:ring-2 focus:ring-blue-500 resize-vertical
                {$description.errors.length > 0 ? 'border-red-500' : 'border-gray-600'}"
        placeholder="Optional description of this node's role or purpose..."
      ></textarea>
      {#if $description.errors.length > 0}
        <div class="flex items-center gap-2 text-red-400">
          <AlertCircle size={16} />
          <p class="text-xs">{$description.errors[0]}</p>
        </div>
      {/if}
      <p class="text-xs text-gray-400">
        Describe what this node does or its role in your infrastructure
      </p>
    </div>
</div>