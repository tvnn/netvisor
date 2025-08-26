<script lang="ts">
	import { getNodeTarget, getNodeTypeDisplay, nodeTargets, nodeTypes } from "$lib/api/registry";
  import type { NodeFormData, NodeTarget } from "$lib/components/nodes/types";
  import TargetConfigForm from './TargetConfigForm.svelte';
  import RichRadioCheck from '../../../common/RichRadioCheck.svelte';
  
  export let formData: NodeFormData;
  export let errors: Record<string, string>;
  export let isEditing: boolean;

  let selectedNodeTypeValue = !isEditing && formData.node_type == 'UnknownDevice' ? "" : formData.node_type;

  $: if (selectedNodeTypeValue !== '') {
    formData.node_type = selectedNodeTypeValue
  }
  
  const nodeTypeSelectOptions = $nodeTypes.map(t => {return {value:t.id, label: t.display_name}});

  $: selectOptions = isEditing 
    ? nodeTypeSelectOptions  
    : [{ value: '', label: 'Please select...' }, ...nodeTypeSelectOptions];

  // Transform node targets into RichRadioCheck format
  $: targetTypeOptions = $nodeTargets.map(target => ({
    id: target.id,
    title: target.display_name,
    description: target.description,
    value: target.id,
    category: target.category,
    metadata: target.metadata
  }));

  // Handle target type changes
  function handleTargetTypeChange(value: string | string[]) {
    const selectedTargetType = typeof value === 'string' ? value : value[0];
    const targetMetadata = $getNodeTarget(selectedTargetType);
    
    formData.target = {
        type: selectedTargetType,
        config: {}
      } as NodeTarget;
    
    // Trigger reactivity
    formData = { ...formData };
  }

</script>

<div class="space-y-4">  
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
        Node Type
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
    <!-- Target Type Selection -->
    <div>
      <label for="target_type" class="block text-sm font-medium text-gray-300 mb-2">
        Connection Method
      </label>
      
      <RichRadioCheck
        mode="radio"
        name="target_type"
        options={targetTypeOptions}
        selectedValue={formData.target.type}
        onChange={handleTargetTypeChange}
        columns={1}
      />
    </div>

    <!-- Target-specific Configuration -->
    <TargetConfigForm 
      bind:target={formData.target}
    />
  </div>
</div>