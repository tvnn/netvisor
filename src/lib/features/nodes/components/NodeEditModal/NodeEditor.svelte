<!-- src/lib/features/nodes/components/NodeEditModal/NodeEditor.svelte -->
<script lang="ts">
  import { Server, Settings, Shield, Info, Network } from 'lucide-svelte';
  import type { Node } from "$lib/features/nodes/types/base";
  import { createEmptyNodeFormData } from "$lib/features/nodes/store";
  import DetailsForm from './Details/DetailsForm.svelte';
  import CapabilitiesForm from './Capabilities/CapabilitiesForm.svelte';
	import EditModal from '$lib/shared/components/forms/EditModal.svelte';
	import SubnetsForm from './Subnets/SubnetsForm.svelte';
  
  export let node: Node | null = null;
  export let isOpen = false;
  export let onCreate: (data: Node) => Promise<void> | void;
  export let onUpdate: (id: string, data: Node) => Promise<void> | void;
  export let onClose: () => void;
  export let onDelete: ((id: string) => Promise<void> | void) | null = null;
  
  let loading = false;
  let deleting = false;
  
  // Tab management
  let activeTab = 'details';
  const tabs = [
    { 
      id: 'details', 
      label: 'Details',
      icon: Info,
      description: 'Basic node information and connection details'
    },
    { 
      id: 'capabilities', 
      label: 'Capabilities',
      icon: Shield,
      description: 'Services and monitoring configuration'
    },
    { 
      id: 'subnets', 
      label: 'Subnets',
      icon: Network,
      description: 'Subnet membership'
    }
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
    activeTab = 'details'; // Reset to first tab
  }
  
  async function handleSubmit() {
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
  
  // Handle form-based submission for create flow with steps
  function handleFormSubmit() {
    if (isEditing || currentTabIndex === tabs.length - 1) {
      handleSubmit();
    } else {
      nextTab();
    }
  }
  
  function handleFormCancel() {
    if (isEditing) {
      onClose();
    } else {
      previousTab();
    }
  }
  
  // Dynamic labels based on create/edit mode and tab position
  $: saveLabel = isEditing ? 'Update Node' : (
    currentTabIndex === tabs.length - 1 ? 'Create Node' : 'Next'
  );
  $: cancelLabel = isEditing ? 'Cancel' : 'Previous';
  $: showCancel = isEditing ? true : activeTab !== 'details';
</script>

<EditModal
  {isOpen}
  {title}
  {loading}
  {deleting}
  {saveLabel}
  {cancelLabel}
  onSave={handleFormSubmit}
  onCancel={showCancel ? handleFormCancel : null}
  onDelete={isEditing ? handleDelete : null}
  size="full"
  let:form
>
  <!-- Header icon -->
  <svelte:fragment slot="header-icon">
    <div class="p-2 bg-blue-600/20 rounded-lg">
      <Server class="w-5 h-5 text-blue-400" />
    </div>
  </svelte:fragment>
  
  <!-- Content -->
  <div class="h-full flex flex-col min-h-0">
    <!-- Tab Navigation (only show for editing) -->
    {#if isEditing}
      <div class="border-b border-gray-700 px-6">
        <nav class="flex space-x-8" aria-label="Node editor tabs">
          {#each tabs as tab}
            <button
              type="button"
              on:click={() => activeTab = tab.id}
              class="py-4 px-1 border-b-2 font-medium text-sm transition-colors
                     {activeTab === tab.id 
                       ? 'border-blue-500 text-blue-400' 
                       : 'border-transparent text-gray-400 hover:text-gray-300 hover:border-gray-600'}"
              aria-current={activeTab === tab.id ? 'page' : undefined}
            >
              <div class="flex items-center gap-2">
                <svelte:component this={tab.icon} class="w-4 h-4" />
                <span>{tab.label}</span>
                
                <!-- Show capability count indicator -->
                {#if tab.id === 'capabilities' && formData.capabilities.length > 0}
                  <span class="ml-1 px-1.5 py-0.5 text-xs bg-blue-600 text-white rounded-full">
                    {formData.capabilities.length}
                  </span>
                {/if}
              </div>
            </button>
          {/each}
        </nav>
      </div>
    {/if}
    
    <!-- Tab content -->
    <div class="flex-1 overflow-hidden min-h-0">
      {#if activeTab === 'details'}
        <div class="h-full overflow-auto">
          <div class="p-6">
            {#if !isEditing}
              <div class="mb-6">
                <h3 class="text-lg font-medium text-white mb-2">Node Details</h3>
                <p class="text-sm text-gray-400">
                  Configure basic information about this node including its connection details and metadata.
                </p>
              </div>
            {/if}
            
            <DetailsForm 
              {form}
              bind:formData={formData}
              {isEditing}
            />
          </div>
        </div>
        
      {:else if activeTab === 'capabilities'}
        <div class="h-full overflow-hidden">
          {#if !isEditing}
            <div class="p-6 pb-4 border-b border-gray-700 flex-shrink-0">
              <h3 class="text-lg font-medium text-white mb-2">Capabilities & Monitoring</h3>
              <p class="text-sm text-gray-400">
                Define what services this node provides and configure monitoring tests for each capability.
              </p>
            </div>
          {/if}
          
          <div class="flex-1 relative">
            <CapabilitiesForm 
              {form}
              bind:selectedCapabilities={formData.capabilities}
              {nodeContext}
            />
          </div>
        </div>
      {:else if activeTab == 'subnets'}
      <div class="h-full overflow-hidden">
          {#if !isEditing}
            <div class="p-6 pb-4 border-b border-gray-700 flex-shrink-0">
              <h3 class="text-lg font-medium text-white mb-2">Subnets</h3>
              <p class="text-sm text-gray-400">
                Select the subnets that this node is a member of.
              </p>
            </div>
          {/if}
          
          <div class="flex-1 relative">
            <SubnetsForm 
              {form}
              bind:formData={formData}/>
          </div>
        </div>
      {/if}
    </div>
  </div>
</EditModal>