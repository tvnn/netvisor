<script lang="ts">
  import type { NodeFormData, Node, NodeApi, AssignedTest, CapabilityRecommendations } from "$lib/types/nodes";
  import { nodeActions } from '$lib/stores/nodes';
  import EditModal from '../../common/EditModal.svelte'
  import BasicNodeForm from './BasicNodeForm.svelte';
  import CapabilitiesForm from './CapabilitiesForm.svelte';
  import TestsForm from './TestsForm.svelte';
  import TestConfigPanel from './TestConfigPanel.svelte';
  
  export let node: Node | null = null;
  export let isOpen = false;
  export let onCreate: (data: any) => Promise<void> | void;
  export let onUpdate: (id: string, data: any) => Promise<void> | void;
  export let onClose: () => void;
  export let onDelete: ((id: string) => Promise<void> | void) | null = null;
  
  let loading = false;
  let deleting = false;
  let errors: Record<string, string> = {};
  let showTestConfig = false;
  let editingTest: AssignedTest | null = null;
  let editingTestIndex: number = -1;
  
  // Tab management
  let activeTab = 'details';
  const tabs = [
    { id: 'details', label: 'Details', icon: 'Info' },
    { id: 'capabilities', label: 'Capabilities', icon: 'Settings' },
    { id: 'tests', label: 'Tests', icon: 'CheckCircle' }
  ];
  
  $: isEditing = node !== null;
  $: title = isEditing ? `Edit ${node?.name}` : 'Create Node';
  
  let formData = createEmptyFormData();
  
  // Initialize form data when node changes or modal opens
  $: if (isOpen) {
    resetForm();
  }

  function createEmptyFormData(): NodeFormData {
    return {
      name: '',
      domain: '',
      ip: '',
      port: 80,
      path: '',
      description: '',
      node_type: 'UnknownDevice',
      capabilities: [],
      monitoring_enabled: true,
      assigned_tests: []
    };
  }

  function nodeToFormData(node: Node): NodeFormData {
    return {
      name: node.name,
      domain: node.domain || '',
      ip: node.ip || '',
      port: node.port || 80,
      path: node.path || '',
      description: node.description || '',
      node_type: node.node_type || 'UnknownDevice',
      capabilities: [...node.capabilities],
      monitoring_enabled: node.monitoring_enabled,
      assigned_tests: [...node.assigned_tests]
    };
  }
  
  function resetForm() {
    formData = node ? nodeToFormData(node) : createEmptyFormData();
    errors = {};
    showTestConfig = false;
    editingTest = null;
    editingTestIndex = -1;
    activeTab = 'details'; // Reset to first tab
  }
  
  function validateForm(): boolean {
    errors = {};
    
    if (!formData.name.trim()) {
      errors.name = 'Name is required';
    }
    
    if (formData.port && (isNaN(Number(formData.port)) || Number(formData.port) < 1 || Number(formData.port) > 65535)) {
      errors.port = 'Port must be between 1 and 65535';
    }
    
    return Object.keys(errors).length === 0;
  }
  
  async function handleSubmit(data: any) {
    if (!validateForm()) {
      return;
    }
    
    const nodeData: NodeApi = {
      name: formData.name.trim(),
      ...(formData.domain.trim() && { domain: formData.domain.trim() }),
      ...(formData.ip.trim() && { ip: formData.ip.trim() }),
      ...(formData.port && { port: Number(formData.port) }),
      ...(formData.path.trim() && { path: formData.path.trim() }),
      ...(formData.description.trim() && { description: formData.description.trim() }),
      ...(formData.node_type !== 'UnknownDevice' && { node_type: formData.node_type }),
      capabilities: formData.capabilities,
      monitoring_enabled: formData.monitoring_enabled,
      assigned_tests: formData.assigned_tests,
      node_groups: [],
      current_status: 'Unknown',
      subnet_membership: [],
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
  
  function handleTestEdit(test: AssignedTest, index: number) {
    editingTest = { ...test };
    editingTestIndex = index;
    showTestConfig = true;
  }
  
  function handleTestCancel() {
    showTestConfig = false;
    editingTest = null;
    editingTestIndex = -1;
  }

  function handleTestChange(updatedTest: AssignedTest) {
    if (editingTestIndex >= 0) {
      formData.assigned_tests[editingTestIndex] = updatedTest;
      formData.assigned_tests = [...formData.assigned_tests]; // Trigger reactivity
    }
  }
  
  function handleTestCreate(newTest: AssignedTest) {
    formData.assigned_tests = [...formData.assigned_tests, newTest];
    showTestConfig = false;
    editingTest = null;
    editingTestIndex = -1;
  }
  
  // Capability recommendations cache
  let capabilityRecommendations: CapabilityRecommendations;
  
  // Auto-load capability recommendations when node type changes
  let lastNodeType = formData.node_type;
  $: if (formData.node_type !== lastNodeType && isOpen) {
    lastNodeType = formData.node_type;
    // Trigger capability recommendation fetch when node type changes
    if (formData.node_type !== 'UnknownDevice') {
      preloadCapabilityRecommendations();
    }
  }
  
  // Preload capability recommendations for better UX
  async function preloadCapabilityRecommendations() {
    try {
      const response = await nodeActions.getCapabilityRecommendations(formData.node_type);
      if (response) {
        capabilityRecommendations = response;
        
        // Auto-apply suggestions if no capabilities are currently selected
        if (formData.capabilities.length === 0 && response.suggested_capabilities.length > 0) {
          formData.capabilities = [...response.suggested_capabilities];
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
  onSubmit={handleSubmit}
  {onClose}
  onDelete={isEditing ? handleDelete : null}
  submitLabel={isEditing ? 'Update Node' : 'Create Node'}
>
  <!-- Tab Navigation -->
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
          {#if tab.id === 'tests' && formData.monitoring_enabled}
            <span class="ml-1 w-2 h-2 bg-green-400 rounded-full inline-block"></span>
          {/if}
        </button>
      {/each}
    </nav>
  </div>

  <!-- Tab Content -->
  <div class="tab-content">
    {#if activeTab === 'details'}
      <!-- Basic Information Tab -->
      <div class="space-y-6">
        <BasicNodeForm 
          bind:formData={formData}
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
        <!-- Monitoring Toggle -->
        <div class="p-4 bg-gray-700/20 border border-gray-600 rounded-lg">
          <div class="flex items-start space-x-3">
            <input
              type="checkbox"
              id="monitoring_enabled"
              bind:checked={formData.monitoring_enabled}
              class="mt-1 rounded bg-gray-700 border-gray-600 text-blue-600 focus:ring-blue-500"
            />
            <div class="flex-1">
              <label for="monitoring_enabled" class="text-sm font-medium text-white cursor-pointer">
                Enable Monitoring
              </label>
              <p class="text-sm text-gray-400 mt-1">
                When enabled, assigned tests will run automatically at their configured intervals.
              </p>
            </div>
          </div>
        </div>
        
        <!-- Tests Configuration -->
        <div class="grid grid-cols-1 gap-6" class:lg:grid-cols-2={showTestConfig}>
          <!-- Tests List -->
          <div class="space-y-6">
            <TestsForm 
              bind:tests={formData.assigned_tests}
              onEditTest={handleTestEdit}
              onCreateTest={() => {
                editingTest = null;
                editingTestIndex = -1;
                showTestConfig = true;
              }}
            />
          </div>
          
          <!-- Test Configuration Panel (slides in when needed) -->
          {#if showTestConfig}
            <div class="border-l border-gray-700 pl-6">
              <TestConfigPanel
                test={editingTest}
                node={formData}
                onChange={editingTestIndex >= 0 ? handleTestChange : handleTestCreate}
                onCancel={handleTestCancel}
              />
            </div>
          {/if}
        </div>
      </div>
      
      <!-- Test Summary -->
      <div class="flex justify-start items-center pt-4 border-t border-gray-600 mt-6">
        <div class="text-sm text-gray-400">
          {#if formData.assigned_tests.length > 0}
            {formData.assigned_tests.length} test{formData.assigned_tests.length === 1 ? '' : 's'} configured
            {#if formData.monitoring_enabled}
              • <span class="text-green-400">{formData.assigned_tests.filter(t => t.enabled && t.monitor_interval_minutes).length} with monitoring intervals</span>
            {/if}
          {:else}
            No tests configured yet
          {/if}
        </div>
      </div>
    {/if}
  </div>
</EditModal>

<style>
  .tab-content {
    min-height: 400px; /* Prevent jumping when switching tabs */
  }
</style>