<script lang="ts">
  import { nodes } from '../../stores/nodes';
  import GenericEditModal from '../common/EditModal.svelte';
  import ListManager from '../common/ListManager.svelte';
  import type { NodeGroupApi, NodeGroup, NodeGroupFormData } from '$lib/types/node-groups'
  
  export let group: NodeGroup | null = null;
  export let isOpen = false;
  export let onCreate: (data: any) => Promise<void> | void;
  export let onUpdate: (id: string, data: any) => Promise<void> | void;
  export let onClose: () => void;
  export let onDelete: ((id: string) => Promise<void> | void) | null = null;
  
  let formData = createEmptyFormData();
  
  let loading = false;
  let deleting = false;
  let errors: Record<string, string> = {};
  
  $: isEditing = group !== null;
  $: title = isEditing ? `Edit ${group?.name}` : 'Create Node Group';
  
  // Initialize form data when group changes or modal opens
  $: if (isOpen) {
    resetForm();
  }

  function createEmptyFormData(): NodeGroupFormData {
    return {
      name: '',
      description: '',
      node_sequence: [] as string[],
      auto_diagnostic_enabled: true
    };
  }

  function nodeGroupToFormData(nodeGroup: NodeGroup): NodeGroupFormData {
    return {
      name: nodeGroup.name,
      description: nodeGroup.description || '',
      node_sequence: nodeGroup.node_sequence || [],
      auto_diagnostic_enabled: nodeGroup.auto_diagnostic_enabled || true
    };
  }
  
  function resetForm() {
    formData = group ? nodeGroupToFormData(group) : createEmptyFormData();
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
  
  async function handleSubmit(data: any) {
    const groupData: Partial<NodeGroupApi> = {
      name: formData.name.trim(),
      ...(formData.description.trim() && { description: formData.description.trim() }),
      node_sequence: formData.node_sequence,
      auto_diagnostic_enabled: formData.auto_diagnostic_enabled
    };
    
    if (!validateForm()) {
      return;
    }
    
    loading = true;
    try {
      if (isEditing && group) {
        await onUpdate(group.id, groupData);
      } else {
        await onCreate(groupData);
      }
    } finally {
      loading = false;
    }
  }
  
  async function handleDelete() {
    if (onDelete && group) {
      deleting = true;
      try {
        await onDelete(group.id);
      } finally {
        deleting = false;
      }
    }
  }
  
  function getNodeName(nodeId: string): string {
    const node = $nodes.find(n => n.id === nodeId);
    return node ? node.name : `Node ${nodeId.slice(0, 8)}...`;
  }
</script>

<GenericEditModal
  {isOpen}
  {title}
  {loading}
  {deleting}
  onSubmit={handleSubmit}
  {onClose}
  onDelete={isEditing ? handleDelete : null}
  submitLabel={isEditing ? 'Update Group' : 'Create Group'}
>
  <!-- Basic Information -->
  <div>
    <label for="name" class="block text-sm font-medium text-gray-300 mb-1">
      Group Name *
    </label>
    <input
      id="name"
      name="name"
      bind:value={formData.name}
      type="text"
      required
      placeholder="VPN Access Path"
      class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
      class:border-red-500={errors.name}
    />
    {#if errors.name}
      <p class="text-red-400 text-xs mt-1">{errors.name}</p>
    {/if}
  </div>
  
  <div>
    <label for="description" class="block text-sm font-medium text-gray-300 mb-1">
      Description
    </label>
    <textarea
      id="description"
      name="description"
      bind:value={formData.description}
      rows="3"
      placeholder="Describe the purpose of this diagnostic sequence..."
      class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
    ></textarea>
  </div>
  
  <!-- Auto Diagnostic Toggle -->
  <div>
    <label class="flex items-center space-x-2">
      <input
        type="checkbox"
        name="auto_diagnostic_enabled"
        bind:checked={formData.auto_diagnostic_enabled}
        class="rounded bg-gray-700 border-gray-600 text-blue-600 focus:ring-blue-500"
      />
      <span class="text-sm font-medium text-gray-300">Enable Auto-Diagnostic</span>
    </label>
    <p class="text-xs text-gray-400 mt-1">
      When enabled, this diagnostic will run automatically when any node in the group fails a test
    </p>
  </div>
  
  <!-- Node Sequence Manager -->
  <ListManager
    label="Diagnostic Sequence"
    bind:items={formData.node_sequence}
    availableOptions={$nodes.map(node => ({
      id: node.id,
      label: node.name,
      subtitle: node.ip || node.domain || 'No address'
    }))}
    placeholder="Select a node to add"
    required={true}
    allowReorder={true}
    getDisplayName={getNodeName}
    error={errors.nodes}
  />
  
  <!-- Info about diagnostic sequence -->
  <div class="bg-purple-900/20 border border-purple-500/30 rounded-lg p-3">
    <p class="text-purple-300 text-sm">
      <strong>Diagnostic Sequence:</strong> Tests will be executed on nodes in the order specified above. 
      This allows you to follow logical network paths and dependencies during troubleshooting.
    </p>
  </div>
</GenericEditModal>