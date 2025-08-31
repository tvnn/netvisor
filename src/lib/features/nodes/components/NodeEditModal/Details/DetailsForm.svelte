<script lang="ts">
  import TargetConfigForm from './TargetConfigForm.svelte';
  import type { Node } from '$lib/features/nodes/types/base';
	import { nodeTargets, nodeTypes } from '$lib/shared/stores/registry';
	import type { NodeTarget } from '$lib/features/nodes/types/targets';
	import SubnetListEditor from '$lib/shared/components/forms/SubnetListEditor.svelte';
	import RichRadioCheck from '$lib/shared/components/forms/RichRadioCheck.svelte';
  
  export let formData: Node;
  export let errors: Record<string, string>;
  export let isEditing: boolean;

  let selectedNodeTypeValue = !isEditing && formData.node_type == 'UnknownDevice' ? "" : formData.node_type;

  $: if (selectedNodeTypeValue !== '') {
    formData.node_type = selectedNodeTypeValue
  }
  
  $: nodeTypeSelectOptions = nodeTypes.getItems().map(t => {return {value:t.id, label: t.display_name}});

  $: selectOptions = isEditing 
    ? nodeTypeSelectOptions  
    : [{ value: '', label: 'Please select...' }, ...nodeTypeSelectOptions];

  // Transform node targets into RichRadioCheck format
  $: targetTypeOptions = nodeTargets.getItems().map(target => ({
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
    const targetMetadata = targetTypeOptions.find(n => n.id = selectedTargetType)?.metadata;
    
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

      <!-- Hostname (separate from target config) -->
      <div>
        <label for="hostname" class="block text-sm font-medium text-gray-300 mb-1">
          Hostname
        </label>
        <input
          id="hostname"
          name="hostname"
          bind:value={formData.hostname}
          type="text"
          placeholder="server.local"
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <p class="text-xs text-gray-400 mt-1">Resolved or configured hostname for this node</p>
      </div>

      <!-- MAC Address -->
      <div>
        <label for="mac_address" class="block text-sm font-medium text-gray-300 mb-1">
          MAC Address
        </label>
        <input
          id="mac_address"
          name="mac_address"
          bind:value={formData.mac_address}
          type="text"
          placeholder="00:1B:44:11:3A:B7"
          pattern="^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$"
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <p class="text-xs text-gray-400 mt-1">Physical network address</p>
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

  <!-- Network Subnets -->
  <SubnetListEditor bind:subnets={formData.subnets} />
  
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
          columns={2}
        />
    </div>

    <!-- Target-specific Configuration -->
    <TargetConfigForm 
      bind:target={formData.target}
    />
  </div>
</div>