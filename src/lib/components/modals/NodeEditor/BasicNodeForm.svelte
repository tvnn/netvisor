<script lang="ts">
  import type { NodeType, NodeFormData, NodeTargetType, NodeTarget } from "$lib/types/nodes";
  import { getNodeTypeDisplay, getNodeTypes } from "$lib/config/nodes/types";
  import { getNodeTargetTypes, getNodeTargetTypeDisplayName, getNodeTargetTypeDescription, getNodeTargetTypeDefaultConfig } from "$lib/config/nodes/targets";
  import TargetConfigForm from './TargetConfigForm.svelte';
  
  export let formData: NodeFormData;
  export let errors: Record<string, string>;
  export let isEditing: boolean;

  let selectedNodeTypeValue = !isEditing && formData.node_type == 'UnknownDevice' ? "" : formData.node_type;

  $: if (selectedNodeTypeValue !== '') {
    formData.node_type = selectedNodeTypeValue as NodeType;
  }
  
  const nodeTypeSelectOptions = getNodeTypes().map(t => {return {value:t, label: getNodeTypeDisplay(t)}});
  const targetTypes = getNodeTargetTypes();

  $: selectOptions = isEditing 
    ? nodeTypeSelectOptions  
    : [{ value: '', label: 'Please select...' }, ...nodeTypeSelectOptions];

  // Handle target type changes
  function handleTargetTypeChange(newTargetType: NodeTargetType) {
    formData.target = {
      type: newTargetType,
      config: getNodeTargetTypeDefaultConfig(newTargetType)
    } as NodeTarget;
  }

</script>

<div class="space-y-4">
  <h3 class="text-lg font-medium text-white">Basic Information</h3>
  
  <!-- Name and Node Type -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <div>
      <label for="name" class="block text-sm font-medium text-gray-300 mb-1">
        Name *
      </label>
      <input
        id="name"
        name="name"
        bind:value={formData.name}
        type="text"
        required
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        class:border-red-500={errors.name}
      />
      {#if errors.name}
        <p class="text-red-400 text-xs mt-1">{errors.name}</p>
      {/if}
    </div>
    
    <div>
      <label for="node_type" class="block text-sm font-medium text-gray-300 mb-1">
        Type
      </label>
      <select
        id="node_type"
        name="node_type"
        required
        bind:value={selectedNodeTypeValue}
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        {#each selectOptions as option}
          <option value={option.value} disabled={option.value === ''}>{option.label}</option>
        {/each}
      </select>
      {#if errors.node_type}
        <p class="text-red-400 text-xs mt-1">{errors.node_type}</p>
      {/if}
    </div>
  </div>
  
  <!-- Target Configuration Section -->
  <div class="space-y-4">
    <div>
      <h4 class="text-base font-medium text-white mb-2">How to Reach This Node</h4>
      <p class="text-sm text-gray-400 mb-4">
        Specify how NetFrog should connect to this node for testing. Choose one method:
      </p>
    </div>

    <!-- Target Type Selection -->
    <div>
      <label for="target_type" class="block text-sm font-medium text-gray-300 mb-2">
        Connection Method
      </label>
      <div class="space-y-2">
        {#each targetTypes as targetType}
          <label class="flex items-start space-x-3 p-3 bg-gray-700/30 border border-gray-600 rounded-lg cursor-pointer hover:bg-gray-700/50 transition-colors">
            <input
              type="radio"
              name="target_type"
              value={targetType}
              checked={formData.target.type === targetType}
              on:change={() => handleTargetTypeChange(targetType)}
              class="mt-1 text-blue-600 bg-gray-700 border-gray-600 focus:ring-blue-500"
            />
            <div class="flex-1">
              <div class="text-sm font-medium text-white">
                {getNodeTargetTypeDisplayName(targetType)}
              </div>
              <div class="text-xs text-gray-400 mt-1">
                {getNodeTargetTypeDescription(targetType)}
              </div>
            </div>
          </label>
        {/each}
      </div>
    </div>

    <!-- Target-specific Configuration -->
    <TargetConfigForm 
      bind:target={formData.target}
    />
  </div>
  
  <!-- Description -->
  <div>
    <label for="description" class="block text-sm font-medium text-gray-300 mb-1">
      Description
    </label>
    <textarea
      id="description"
      name="description"
      bind:value={formData.description}
      rows="3"
      placeholder="Optional description..."
      class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
    ></textarea>
  </div>

  <!-- Monitoring Configuration -->
  <div class="space-y-4">
    <h4 class="text-base font-medium text-white">Monitoring Configuration</h4>
    
    <div>
      <label for="monitoring_interval" class="block text-sm font-medium text-gray-300 mb-1">
        Monitoring Interval (minutes)
      </label>
      <input
        id="monitoring_interval"
        name="monitoring_interval"
        bind:value={formData.monitoring_interval}
        type="number"
        min="0"
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        placeholder="10"
      />
      <p class="text-xs text-gray-400 mt-1">
        Set to 0 to disable monitoring, or specify interval in minutes. Critical infrastructure should be monitored every 5 minutes.
      </p>
    </div>
  </div>
</div>