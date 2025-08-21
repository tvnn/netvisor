<script lang="ts">
  import Monitoring from './Tests/Monitoring.svelte';

  import type { Node, AssignedTest } from "$lib/types/nodes";
  import { createEmptyNodeFormData, nodeToFormData, formDataToNodeApi } from "$lib/types/nodes";
  import { nodeActions } from '$lib/stores/nodes';
  import EditModal from '../../common/EditModal.svelte'
  import BasicNodeForm from './Details/DetailsForm.svelte';
  import CapabilitiesForm from './Capabilities/CapabilitiesForm.svelte';
  import TestsForm from './Tests/TestsForm.svelte';
  import TestConfigPanel from './Tests/TestConfigPanel.svelte';
  
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
</script>

<EditModal
  {isOpen}
  {title}
  {loading}
  {deleting}
  onSubmit={ isEditing || currentTabIndex == tabs.length-1 ? handleSubmit : nextTab }
  {onClose}
  showCancel={isEditing ? true : activeTab != 'details'}
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
  <div class="tab-content space-y-6">
    {#if activeTab === 'details'}
      {#if !isEditing}
        <h3 class="text-lg font-medium text-white mb-4">Details</h3>
      {/if}
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
      {#if !isEditing}
        <h3 class="text-lg font-medium text-white mb-4">Capabilities</h3>
      {/if}
      <CapabilitiesForm 
        bind:selectedCapabilities={formData.capabilities}
        nodeType={formData.node_type || 'UnknownDevice'}
      />
      
    {:else if activeTab === 'tests'}
      <!-- Tests Tab -->
      {#if !isEditing}
        <h3 class="text-lg font-medium text-white mb-4">Tests</h3>
      {/if}
      <Monitoring formData={formData} />
      <!-- Tests List and Editor -->
      <div class="space-y-4">
        <TestsForm 
          bind:tests={formData.assigned_tests}
          node={formData}
        />
      </div>
    {/if}
  </div>
</EditModal>