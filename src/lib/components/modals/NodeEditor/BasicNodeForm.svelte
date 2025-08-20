<script lang="ts">
	import { getNodeTarget, getNodeTypeDisplay, nodeTargets, nodeTypes } from "$lib/api/registry";
  import type { NodeFormData, NodeTarget } from "$lib/types/nodes";
  import TargetConfigForm from './TargetConfigForm.svelte';
  
  export let formData: NodeFormData;
  export let errors: Record<string, string>;
  export let isEditing: boolean;

  let selectedNodeTypeValue = !isEditing && formData.node_type == 'UnknownDevice' ? "" : formData.node_type;

  $: if (selectedNodeTypeValue !== '') {
    formData.node_type = selectedNodeTypeValue
  }
  
  const nodeTypeSelectOptions = $nodeTypes.map(t => {return {value:t.id, label: t.display_name}});
  const nodeTargetTypeRadioOptions = $nodeTargets

  $: selectOptions = isEditing 
    ? nodeTypeSelectOptions  
    : [{ value: '', label: 'Please select...' }, ...nodeTypeSelectOptions];

  // Handle target type changes
  function handleTargetTypeChange(newTargetType: string) {
    formData.target = {
      type: newTargetType,
      config: $getNodeTarget(newTargetType)?.metadata['default_config'],
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
        {#each nodeTargetTypeRadioOptions as targetType}
          <label class="flex items-start space-x-3 p-3 bg-gray-700/30 border border-gray-600 rounded-lg cursor-pointer hover:bg-gray-700/50 transition-colors">
            <input
              type="radio"
              name="target_type"
              value={targetType}
              checked={formData.target.type === targetType.id}
              on:change={() => handleTargetTypeChange(targetType.id)}
              class="mt-1 text-blue-600 bg-gray-700 border-gray-600 focus:ring-blue-500"
            />
            <div class="flex-1">
              <div class="text-sm font-medium text-white">
                {targetType.display_name}
              </div>
              <div class="text-xs text-gray-400 mt-1">
                {targetType.display_name}
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
</div>