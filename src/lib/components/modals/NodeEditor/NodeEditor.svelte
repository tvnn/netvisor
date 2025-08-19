<script lang="ts">
  import type { NodeFormData, Node, AssignedTest, NodeCapability } from "$lib/types/nodes";
  import { createEmptyNodeFormData, nodeToFormData, formDataToNodeApi } from "$lib/types/nodes";
  import { nodeActions } from '$lib/stores/nodes';
  import EditModal from '../../common/EditModal.svelte'
  import BasicNodeForm from './BasicNodeForm.svelte';
  import CapabilitiesForm from './CapabilitiesForm.svelte';
  import TestsForm from './TestsForm.svelte';
  import TestConfigPanel from './TestConfigPanel.svelte';
	import { validateTarget } from "$lib/config/nodes/targets";
  
  export let node: Node | null = null;
  export let isOpen = false;
  export let onCreate: (data: any) => Promise<void> | void;
  export let onUpdate: (id: string, data: any) => Promise<void> | void;
  export let onClose: () => void;
  export let onDelete: ((id: string) => Promise<void> | void) | null = null;
  
  let loading = false;
  let deleting = false;
  let errors: Record<string, string> = {};
  let editingTest: AssignedTest | null = null;
  let editingTestIndex: number = -1;
  
  // Tab management
  let activeTab = 'details';
  const tabs = [
    { id: 'details', label: 'Details', icon: 'Info' },
    { id: 'capabilities', label: 'Capabilities', icon: 'Settings' },
    { id: 'tests', label: 'Tests', icon: 'CheckCircle' }
  ];

  $: currentTabIndex = tabs.findIndex(t => t.id == activeTab) || 0

  function nextTab() {
    if (currentTabIndex < tabs.length-1) activeTab = tabs[currentTabIndex+1].id;
  }

  function previousTab() {
    if (currentTabIndex > 0) activeTab = tabs[currentTabIndex-1].id;
  }
  
  $: isEditing = node !== null;
  $: title = isEditing ? `Edit ${node?.name}` : 'Create Node';
  
  let formData = createEmptyNodeFormData();
  
  // Initialize form data when node changes or modal opens
  $: if (isOpen) {
    resetForm();
  }
  
  function resetForm() {
    formData = node ? nodeToFormData(node) : createEmptyNodeFormData();
    errors = {};
    editingTest = null;
    editingTestIndex = -1;
    activeTab = 'details'; // Reset to first tab
  }
  
  function validateForm(): boolean {
    errors = {};
    
    if (!formData.name.trim()) {
      errors.name = 'Name is required';
    }
    
    return Object.keys(errors).length === 0;
  }
  
  async function handleSubmit(data: any) {
    if (!validateForm()) {
      return;
    }
    
    const nodeData = formDataToNodeApi(formData);
    
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
  
  function handleTestEdit(test: AssignedTest, index: number) {
    editingTest = { ...test };
    editingTestIndex = index;
  }
  
  function handleTestCancel() {
    editingTest = null;
    editingTestIndex = -1;
  }

  function handleTestChange(updatedTest: AssignedTest) {
    if (editingTestIndex >= 0) {
      // Create a new array to trigger reactivity
      const newTests = [...formData.assigned_tests];
      newTests[editingTestIndex] = updatedTest;
      formData.assigned_tests = newTests;
      
      // Also update the editing test to keep the panel in sync
      editingTest = { ...updatedTest };
    }
  }
  
  function handleTestCreate(newTest: AssignedTest) {
    formData.assigned_tests = [...formData.assigned_tests, newTest];
    editingTest = { ...newTest };
    editingTestIndex = formData.assigned_tests.length - 1;
  }
  
  // Capability recommendations cache
  let capabilityRecommendations: NodeCapability[] = [];
  
  // Auto-load capability recommendations when node type changes
  let lastNodeType = formData.node_type;
  $: if (formData.node_type !== lastNodeType && isOpen) {
    lastNodeType = formData.node_type;
    // Trigger capability recommendation fetch when node type changes
    if (formData.node_type !== 'UnknownDevice') {
      preloadCapabilityCompatibility();
    }
  }
  
  // Preload capability recommendations for better UX
  async function preloadCapabilityCompatibility() {
    try {
      const response = await nodeActions.getCapabilityCompatibility(formData.node_type);
      if (response) {
        capabilityRecommendations = response.recommendations || [];
        
        // Auto-apply suggestions if no capabilities are currently selected
        if (formData.capabilities.length === 0 && response.recommendations && response.recommendations.length > 0) {
          formData.capabilities = [...response.recommendations];
        }
      }
    } catch (error) {
      console.error('Failed to preload capability recommendations:', error);
    }
  }
</script>

<EditModal
  {isOpen}
  {title}
  {loading}
  {deleting}
  onSubmit={ isEditing || currentTabIndex == tabs.length-1 ? handleSubmit : nextTab }
  {onClose}
  onCancel={isEditing ? onClose : previousTab}
  cancelLabel={isEditing ? 'Cancel' : 'Previous'}
  onDelete={isEditing ? handleDelete : null}
  submitLabel={isEditing ? 'Update Node' : (
    currentTabIndex == tabs.length-1 ? 'Create Node' : 'Next'
  )}
>
  <!-- Tab Navigation -->
  {#if isEditing}
  <div class="border-b border-gray-600 mb-6">
    <nav class="flex space-x-8">
      {#each tabs as tab, index}
        <button
          type="button"
          disabled={!isEditing}
          class="py-2 px-1 border-b-2 font-medium text-sm transition-colors duration-200 {
              activeTab === tab.id
                ? 'border-blue-500 text-blue-400'
                : 'border-transparent text-gray-400 hover:text-gray-300 hover:border-gray-300'
          }"
          on:click={() => activeTab = tab.id}
        >
          {tab.label}
          
          <!-- Show indicator for validation errors -->
          {#if tab.id === 'details' && Object.keys(errors).length > 0}
            <span class="ml-1 w-2 h-2 bg-red-400 rounded-full inline-block"></span>
          {/if}
          
          <!-- Show test count indicator -->
          {#if tab.id === 'tests' && formData.assigned_tests.length > 0}
            <span class="ml-1 px-1.5 py-0.5 text-xs bg-blue-600 text-white rounded-full">
              {formData.assigned_tests.length}
            </span>
          {/if}
          
          <!-- Show monitoring status indicator on tests tab -->
          {#if tab.id === 'tests' && formData.monitoring_interval > 0 && isEditing}
            <span class="ml-1 w-2 h-2 bg-green-400 rounded-full inline-block"></span>
          {/if}
        </button>
      {/each}
    </nav>
  </div>
  {/if}

  <!-- Tab Content -->
  <div class="tab-content">
    {#if activeTab === 'details'}
      <!-- Basic Information Tab -->
      <div class="space-y-6">
        <BasicNodeForm 
          bind:formData={formData}
          {isEditing}
          {errors}
        />
      </div>
      
    {:else if activeTab === 'capabilities'}
      <!-- Capabilities Tab -->
      <div class="space-y-6">
        <CapabilitiesForm 
          bind:capabilities={formData.capabilities}
          nodeType={formData.node_type || 'UnknownDevice'}
          nodeId={node?.id}
          preloadedRecommendations={capabilityRecommendations}
        />
      </div>
      
    {:else if activeTab === 'tests'}
      <!-- Tests Tab -->
      <div class="space-y-6">
        <!-- Monitoring Status Display -->
        <div class="p-4 bg-gray-700/20 border border-gray-600 rounded-lg">
          <div class="flex items-start space-x-3">
            <div class="flex-1">
              <h4 class="text-sm font-medium text-white">
                Monitoring Status
              </h4>
              <p class="text-sm text-gray-400 mt-1">
                {#if formData.monitoring_interval > 0}
                  ✅ Monitoring enabled: tests run every {formData.monitoring_interval} minutes
                {:else}
                  ⚠️ Monitoring disabled: tests run only during diagnostics
                {/if}
              </p>
              <p class="text-xs text-gray-500 mt-1">
                Configure monitoring interval in the Details tab.
              </p>
            </div>
          </div>
        </div>

        <!-- Tests List and Editor -->
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <div class="space-y-4">
            <TestsForm 
              bind:tests={formData.assigned_tests}
              editingIndex={editingTestIndex}
              onEditTest={handleTestEdit}
              onCreateTest={() => {
                editingTest = null;
                editingTestIndex = -1;
              }}
            />
          </div>
          
          <div class="space-y-4">
            {#if editingTest !== null || editingTestIndex === -1}
              <TestConfigPanel 
                test={editingTest}
                node={formData}
                onCancel={handleTestCancel}
                onChange={editingTest ? handleTestChange : handleTestCreate}
              />
            {/if}
          </div>
        </div>
      </div>
    {/if}
  </div>
</EditModal>