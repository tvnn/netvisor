<script lang="ts">
  import { onMount } from 'svelte';
  import { Globe, Server, FileText, Tag as TagIcon, AlertCircle } from 'lucide-svelte';
  import { field } from 'svelte-forms';
  import { required } from 'svelte-forms/validators';
  import type { Node } from '$lib/features/nodes/types/base';
  import type { NodeTarget } from '$lib/features/nodes/types/targets';
  import { maxLength, validNodeType } from '$lib/shared/components/forms/validators';
  import TargetConfigForm from './TargetConfigForm.svelte';
	import { nodeTypes } from '$lib/shared/stores/registry';
  
  export let form: any;
  export let formData: Node;
  export let isEditing: boolean = false;
  
  // Create form fields with validation
  const name = field('name', formData.name, [required(), maxLength(100)]);
  const description = field('description', formData.description || '', [maxLength(500)]);
  const nodeType = field('node_type', formData.node_type, [validNodeType(isEditing)]);
  const hostname = field('hostname', formData.hostname || '');
  const macAddress = field('mac_address', formData.mac_address || '');
  
  // Add fields to form
  onMount(() => {
    form.name = name
    form.description = description
    form.nodeType = nodeType
    form.hostname = hostname
    form.macAddress = macAddress
  });
  
  // Update formData when field values change
  $: formData.name = $name.value;
  $: formData.description = $description.value;
  $: formData.node_type = $nodeType.value;
  $: formData.hostname = $hostname.value;
  $: formData.mac_address = $macAddress.value;

  $: nodeTypeOptions = nodeTypes.getItems().map(t => {return {value:t.id, label: t.display_name}});
  
  // Target type options
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
  <!-- Basic Information -->
  <div>
    <h4 class="text-md font-medium text-white mb-4 flex items-center gap-2">
      <FileText class="w-5 h-5" />
      Basic Information
    </h4>
    
    <div class="grid grid-cols-1 gap-6">
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
  </div>
  
  <!-- Connection Target -->
  <div>
    <h4 class="text-md font-medium text-white mb-4 flex items-center gap-2">
      <TagIcon class="w-5 h-5" />
      Connection Target
    </h4>
    
    <div class="space-y-6">
      <!-- Target Type Selection -->
      <div class="space-y-2">
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
          How should NetVisor connect to this node?
        </p>
      </div>
      
      <!-- Target Type Description -->
      {#each targetTypes as targetType}
        {#if targetType.value === formData.target?.type}
          <div class="flex items-start gap-3 p-4 bg-blue-900/20 border border-blue-600/30 rounded-lg">
            <svelte:component this={targetType.icon} class="w-5 h-5 text-blue-400 mt-0.5" />
            <div>
              <h5 class="text-sm font-medium text-blue-300">{targetType.label}</h5>
              <p class="text-sm text-blue-200/80">{targetType.description}</p>
            </div>
          </div>
        {/if}
      {/each}
      
      <!-- Target Configuration -->
      {#if formData.target}
        <TargetConfigForm {form} bind:target={formData.target} />
      {/if}
    </div>
  </div>
  
  <!-- Metadata -->
  <div>
    <h4 class="text-md font-medium text-white mb-4 flex items-center gap-2">
      <Server class="w-5 h-5" />
      Metadata
    </h4>
    
    <div class="grid grid-cols-2 gap-6">
      <!-- Node Type -->
      <div class="space-y-2">
        <label for="node_type" class="block text-sm font-medium text-gray-300">
          Node Type
          {#if !isEditing}
            <span class="text-red-400 ml-1">*</span>
          {/if}
        </label>
        <select
          id="node_type"
          bind:value={$nodeType.value}
          class="w-full px-3 py-2 bg-gray-700 border rounded-md text-white 
                 focus:outline-none focus:ring-2 focus:ring-blue-500
                 {$nodeType.errors.length > 0 ? 'border-red-500' : 'border-gray-600'}"
        >
          {#each nodeTypeOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
        {#if $nodeType.errors.length > 0}
          <div class="flex items-center gap-2 text-red-400">
            <AlertCircle size={16} />
            <p class="text-xs">{$nodeType.errors[0]}</p>
          </div>
        {/if}
        <p class="text-xs text-gray-400">
          Classification for this node type
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

      <!-- Mac Address -->
      <div class="space-y-2">
        <label for="mac_address" class="block text-sm font-medium text-gray-300">
          MAC Address
        </label>
        <input
          id="mac_address"
          type="text"
          bind:value={$macAddress.value}
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white 
                 focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder=""
        />
        <p class="text-xs text-gray-400">
          Physical hardware identifier of this device
        </p>
      </div>
    </div>
  </div>
</div>