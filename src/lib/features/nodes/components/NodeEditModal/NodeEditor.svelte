<script lang="ts">
  import type { Node } from "$lib/features/nodes/types/base";
  import { createEmptyNodeFormData } from "$lib/features/nodes/store";
  import EditModal from '$lib/shared/components/forms/EditModal.svelte';
  import DetailsForm from './Details/DetailsForm.svelte';
  import CapabilitiesForm from './Capabilities/CapabilitiesForm.svelte';
	import Monitoring from "./Details/Monitoring.svelte";
  
  export let node: Node | null = null;
  export let isOpen = false;
  export let onCreate: (data: Node) => Promise<void> | void;
  export let onUpdate: (id: string, data: Node) => Promise<void> | void;
  export let onClose: () => void;
  export let onDelete: ((id: string) => Promise<void> | void) | null = null;
  
  let loading = false;
  let deleting = false;
  let errors: Record<string, string> = {};
  
  // Tab management - only details and capabilities
  let activeTab = 'details';
  const tabs = [
    { id: 'details', label: 'Details' },
    { id: 'capabilities', label: 'Capabilities' }
  ];

  $: currentTabIndex = tabs.findIndex(t => t.id === activeTab) || 0;

  function nextTab() {
    if (currentTabIndex < tabs.length - 1) {
      activeTab = tabs[currentTabIndex + 1].id;
    }
  }

  function previousTab() {
    if (currentTabIndex > 0) {
      activeTab = tabs[currentTabIndex - 1].id;
    }
  }
  
  $: isEditing = node !== null;
  $: title = isEditing ? `Edit ${node?.name}` : 'Create Node';
  

  let formData: Node = createEmptyNodeFormData();
  
  // Initialize form data when node changes or modal opens
  $: if (isOpen) {
    resetForm();
  }
  
  function resetForm() {
    // Work directly with Node - no conversion needed
    formData = node ? { ...node } : createEmptyNodeFormData();
    errors = {};
    activeTab = 'details'; // Reset to first tab
  }
  
  function validateForm(): boolean {
    errors = {};
    
    if (!formData.name.trim()) {
      errors.name = 'Name is required';
    }
    
    if (!formData.node_type || formData.node_type === 'UnknownDevice') {
      if (!isEditing) { // Only require for new nodes
        errors.node_type = 'Node type is required';
      }
    }
    
    return Object.keys(errors).length === 0;
  }
  
  async function handleSubmit() {
    if (!validateForm()) {
      return;
    }
    
    // Clean up the data before sending
    const nodeData: Node = {
      ...formData,
      name: formData.name.trim(),
      description: formData.description?.trim() || '',
    };
    
    loading = true;
    try {
      if (isEditing && node) {
        await onUpdate(node.id, nodeData);
      } else {
        await onCreate(nodeData);
      }
    } finally {
      loading = false;
    }
  }
  
  async function handleDelete() {
    if (onDelete && node) {
      deleting = true;
      try {
        await onDelete(node.id);
      } finally {
        deleting = false;
      }
    }
  }

  // Create node context for capabilities form
  $: nodeContext = {
    node_id: formData.id || undefined,
    node_type: formData.node_type,
    capabilities: formData.capabilities,
    target: formData.target
  };
</script>

<EditModal
  {isOpen}
  {title}
  {loading}
  {deleting}
  onSubmit={isEditing || currentTabIndex === tabs.length - 1 ? handleSubmit : nextTab}
  {onClose}
  showCancel={isEditing ? true : activeTab !== 'details'}
  onCancel={isEditing ? onClose : previousTab}
  cancelLabel={isEditing ? 'Cancel' : 'Previous'}
  onDelete={isEditing ? handleDelete : null}
  submitLabel={isEditing ? 'Update Node' : (
    currentTabIndex === tabs.length - 1 ? 'Create Node' : 'Next'
  )}
>
  <!-- Tab Navigation -->
  {#if isEditing}
    <div class="border-b border-gray-600 mb-6">
      <nav class="flex space-x-8">
        {#each tabs as tab}
          <button
            type="button"
            class="py-2 px-1 border-b-2 font-medium text-sm transition-colors duration-200 {
                activeTab === tab.id
                  ? 'border-blue-500 text-blue-400'
                  : 'border-transparent text-gray-400 hover:text-gray-300 hover:border-gray-300'
            }"
            on:click={() => activeTab = tab.id}
          >
            {tab.label}
            
            <!-- Show indicator for validation errors on details tab -->
            {#if tab.id === 'details' && Object.keys(errors).length > 0}
              <span class="ml-1 w-2 h-2 bg-red-400 rounded-full inline-block"></span>
            {/if}
            
            <!-- Show capability count indicator -->
            {#if tab.id === 'capabilities' && formData.capabilities.length > 0}
              <span class="ml-1 px-1.5 py-0.5 text-xs bg-blue-600 text-white rounded-full">
                {formData.capabilities.length}
              </span>
            {/if}
          </button>
        {/each}
      </nav>
    </div>
  {/if}

  <!-- Tab Content -->
  <div class="tab-content space-y-6">
    {#if activeTab === 'details'}
      {#if !isEditing}
        <h3 class="text-lg font-medium text-white mb-4">Node Details</h3>
      {/if}
      
      <DetailsForm 
        bind:formData={formData}
        {isEditing}
        {errors}
      />
      
    {:else if activeTab === 'capabilities'}
      {#if !isEditing}
        <h3 class="text-lg font-medium text-white mb-4">Capabilities & Monitoring</h3>
      {/if}
      
      <!-- Capabilities Section -->
      <div>
        <h4 class="text-sm font-medium text-gray-300 mb-4">Service Capabilities</h4>
        <CapabilitiesForm 
          bind:selectedCapabilities={formData.capabilities}
          {nodeContext}
        />
      </div>
    {/if}
  </div>
</EditModal>