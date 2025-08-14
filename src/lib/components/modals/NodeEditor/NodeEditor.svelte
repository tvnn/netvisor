<!-- src/lib/components/modals/NodeEditor.svelte -->
<script lang="ts">
  import type { NodeFormData, Node, NodeApi, AssignedTest } from "$lib/types/nodes";
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
      monitoring_enabled: false,
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
    
    // Convert form data to NodeBase, only including fields that have values
    const nodeData: Partial<NodeApi> = {
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
      // Backend will handle these fields:
      // - node_groups: managed separately via node group assignments
      // - current_status: computed from test results
      // - subnet_membership: computed from network discovery
      // - last_seen: managed by monitoring system
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
  <div class="grid grid-cols-1 gap-6" class:lg:grid-cols-2={showTestConfig}>
    <!-- Main Form -->
    <div class="space-y-6">
      <!-- Basic Information -->
      <BasicNodeForm 
        bind:formData={formData}
        {errors}
      />
      
      <!-- Capabilities -->
      <CapabilitiesForm 
        bind:capabilities={formData.capabilities}
        nodeType={formData.node_type || 'UnknownDevice'}
      />
      
      <!-- Tests -->
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
    
    <!-- Test Configuration Panel -->
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
</EditModal>