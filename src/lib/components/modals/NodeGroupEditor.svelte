<script lang="ts">
  import { X, ArrowUp, ArrowDown, Trash2 } from 'lucide-svelte';
  import { nodes } from '../../stores/nodes';
  import type { NodeGroup } from '../../stores/node-groups';
  
  export let group: NodeGroup | null = null;
  export let isOpen = false;
  export let onCreate: (data: any) => void = () => {};
  export let onUpdate: (id: string, data: any) => void = () => {};
  export let onClose: () => void = () => {};
  
  let formData = {
    name: '',
    description: '',
    node_sequence: [] as string[],
    auto_diagnostic_enabled: true
  };
  
  let selectedNodeId = '';
  let loading = false;
  let errors: Record<string, string> = {};
  
  $: isEditing = group !== null;
  $: title = isEditing ? `Edit ${group?.name}` : 'Create Node Group';
  $: availableNodes = $nodes.filter(node => !formData.node_sequence.includes(node.id));
  
  // Initialize form data when group changes or modal opens
  $: if (isOpen) {
    resetForm();
  }
  
  function resetForm() {
    if (group) {
      formData = {
        name: group.name,
        description: group.description,
        node_sequence: [...group.node_sequence],
        auto_diagnostic_enabled: group.auto_diagnostic_enabled
      };
    } else {
      formData = {
        name: '',
        description: '',
        node_sequence: [],
        auto_diagnostic_enabled: true
      };
    }
    selectedNodeId = '';
    errors = {};
  }
  
  function validateForm(): boolean {
    errors = {};
    
    if (!formData.name.trim()) {
      errors.name = 'Name is required';
    }
    
    if (formData.node_sequence.length === 0) {
      errors.nodes = 'At least one node is required';
    }
    
    return Object.keys(errors).length === 0;
  }
  
  function addNode() {
    if (selectedNodeId && !formData.node_sequence.includes(selectedNodeId)) {
      formData.node_sequence = [...formData.node_sequence, selectedNodeId];
      selectedNodeId = '';
    }
  }
  
  function removeNode(nodeId: string) {
    formData.node_sequence = formData.node_sequence.filter(id => id !== nodeId);
  }
  
  function moveNodeUp(index: number) {
    if (index > 0) {
      const newSequence = [...formData.node_sequence];
      [newSequence[index - 1], newSequence[index]] = [newSequence[index], newSequence[index - 1]];
      formData.node_sequence = newSequence;
    }
  }
  
  function moveNodeDown(index: number) {
    if (index < formData.node_sequence.length - 1) {
      const newSequence = [...formData.node_sequence];
      [newSequence[index], newSequence[index + 1]] = [newSequence[index + 1], newSequence[index]];
      formData.node_sequence = newSequence;
    }
  }
  
  function getNodeName(nodeId: string): string {
    const node = $nodes.find(n => n.id === nodeId);
    return node ? node.name : `Node ${nodeId.slice(0, 8)}...`;
  }
  
  async function handleSubmit() {
    if (!validateForm()) return;
    
    loading = true;
    
    try {
      const requestData = {
        name: formData.name.trim(),
        description: formData.description.trim() || '',
        node_sequence: formData.node_sequence,
        auto_diagnostic_enabled: formData.auto_diagnostic_enabled
      };
      
      if (isEditing && group) {
        onUpdate(group.id, requestData);
      } else {
        onCreate(requestData);
      }
    } catch (error) {
      console.error('Form submission error:', error);
    } finally {
      loading = false;
    }
  }
  
  function handleClose() {
    resetForm();
    onClose();
  }
  
  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      handleClose();
    }
  }
</script>

{#if isOpen}
  <div 
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    on:click={handleBackdropClick}
    on:keydown={(e) => e.key === 'Escape' && handleClose()}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto">
      <!-- Header -->
      <div class="flex items-center justify-between p-6 border-b border-gray-700">
        <h2 class="text-xl font-semibold text-white">{title}</h2>
        <button
          on:click={handleClose}
          class="text-gray-400 hover:text-white transition-colors"
        >
          <X class="w-6 h-6" />
        </button>
      </div>
      
      <!-- Form -->
      <form on:submit|preventDefault={handleSubmit} class="p-6 space-y-4">
        <!-- Basic Information -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label for="name" class="block text-sm font-medium text-gray-300 mb-1">
              Group Name *
            </label>
            <input
              id="name"
              bind:value={formData.name}
              type="text"
              class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              class:border-red-500={errors.name}
              placeholder="Enter group name"
              required
            />
            {#if errors.name}
              <p class="text-red-400 text-xs mt-1">{errors.name}</p>
            {/if}
          </div>
          
          <div class="flex items-center">
            <input
              id="auto_diagnostic_enabled"
              bind:checked={formData.auto_diagnostic_enabled}
              type="checkbox"
              class="w-4 h-4 text-blue-600 bg-gray-700 border-gray-600 rounded focus:ring-blue-500"
            />
            <label for="auto_diagnostic_enabled" class="ml-2 text-sm text-gray-300">
              Enable auto-diagnostic
            </label>
          </div>
        </div>
        
        <!-- Description -->
        <div>
          <label for="description" class="block text-sm font-medium text-gray-300 mb-1">
            Description
          </label>
          <textarea
            id="description"
            bind:value={formData.description}
            rows="3"
            class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="Optional description for this group"
          ></textarea>
        </div>
        
        <!-- Node Sequence -->
        <div>
          <label for="sequence" class="block text-sm font-medium text-gray-300 mb-2">
            Diagnostic Sequence *
          </label>
          
          <!-- Add Node -->
          <div class="flex gap-2 mb-3">
            <select
              bind:value={selectedNodeId}
              class="flex-1 px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="">Select a node to add</option>
              {#each availableNodes as node}
                <option value={node.id}>{node.name} ({node.ip || node.domain || 'No address'})</option>
              {/each}
            </select>
            <button
              type="button"
              on:click={addNode}
              disabled={!selectedNodeId}
              class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Add
            </button>
          </div>
          
          <!-- Current Sequence -->
          {#if formData.node_sequence.length > 0}
            <div class="space-y-2 mb-3">
              {#each formData.node_sequence as nodeId, index}
                <div class="flex items-center gap-2 bg-gray-700/50 rounded-lg p-3">
                  <span class="text-gray-400 font-mono text-sm">{index + 1}.</span>
                  <span class="flex-1 text-white">{getNodeName(nodeId)}</span>
                  
                  <div class="flex items-center gap-1">
                    <button
                      type="button"
                      on:click={() => moveNodeUp(index)}
                      disabled={index === 0}
                      class="p-1 text-gray-400 hover:text-white disabled:opacity-50 disabled:cursor-not-allowed"
                      title="Move up"
                    >
                      <ArrowUp class="w-4 h-4" />
                    </button>
                    
                    <button
                      type="button"
                      on:click={() => moveNodeDown(index)}
                      disabled={index === formData.node_sequence.length - 1}
                      class="p-1 text-gray-400 hover:text-white disabled:opacity-50 disabled:cursor-not-allowed"
                      title="Move down"
                    >
                      <ArrowDown class="w-4 h-4" />
                    </button>
                    
                    <button
                      type="button"
                      on:click={() => removeNode(nodeId)}
                      class="p-1 text-red-400 hover:text-red-300"
                      title="Remove node"
                    >
                      <Trash2 class="w-4 h-4" />
                    </button>
                  </div>
                </div>
              {/each}
            </div>
          {:else}
            <div class="text-center py-8 bg-gray-700/20 rounded-lg border-2 border-dashed border-gray-600">
              <p class="text-gray-400">No nodes in sequence</p>
              <p class="text-gray-500 text-sm mt-1">Add nodes above to define the diagnostic order</p>
            </div>
          {/if}
          
          {#if errors.nodes}
            <p class="text-red-400 text-xs mt-1">{errors.nodes}</p>
          {/if}
          
          <p class="text-xs text-gray-400 mt-2">
            Diagnostics will run tests on nodes in this order. Drag to reorder or use the arrow buttons.
          </p>
        </div>
        
        <!-- Actions -->
        <div class="flex justify-end gap-3 pt-4 border-t border-gray-700">
          <button
            type="button"
            on:click={handleClose}
            class="px-4 py-2 text-gray-300 border border-gray-600 rounded-md hover:bg-gray-700 transition-colors"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={loading}
            class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {#if loading}
              Saving...
            {:else}
              {isEditing ? 'Update' : 'Create'} Group
            {/if}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}