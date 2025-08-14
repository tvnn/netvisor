<!-- src/lib/components/modals/NodeEditor.svelte -->
<script lang="ts">
  import type { Node, NodeType, NodeCapability } from "$lib/types/nodes";
  import type { TestType } from "$lib/types/tests";
  import { getNodeTypeDisplayName } from "$lib/types/nodes";
  import { getTestTypeDisplayName } from "$lib/types/tests";
  import GenericEditModal from '../common/EditModal.svelte';
  import ListManager from '../common/ListManager.svelte';
  
  export let node: Node | null = null;
  export let isOpen = false;
  export let onCreate: (data: any) => Promise<void> | void;
  export let onUpdate: (id: string, data: any) => Promise<void> | void;
  export let onClose: () => void;
  export let onDelete: ((id: string) => Promise<void> | void) | null = null;
  
  let formData = {
    name: '',
    domain: '',
    ip: '',
    port: '',
    path: '',
    description: '',
    node_type: '' as NodeType | '',
    capabilities: [] as NodeCapability[],
    monitoring_enabled: false,
    assigned_tests: [] as string[]
  };
  
  let loading = false;
  let deleting = false;
  let errors: Record<string, string> = {};
  
  $: isEditing = node !== null;
  $: title = isEditing ? `Edit ${node?.name}` : 'Create Node';
  
  // Available options for dropdowns
  const nodeTypes: NodeType[] = [
    'Router', 'Switch', 'AccessPoint', 'Firewall',
    'WebServer', 'DatabaseServer', 'MediaServer', 'DnsServer', 'VpnServer', 'NasDevice',
    'Workstation', 'IotDevice', 'Printer', 'Camera', 'UnknownDevice'
  ];
  
  const nodeCapabilities: NodeCapability[] = [
    'SshAccess', 'RdpAccess', 'VncAccess',
    'HttpService', 'HttpsService', 'DatabaseService',
    'DnsService', 'EmailService', 'FtpService'
  ];
  
  const testTypes: TestType[] = [
    'Connectivity', 'DirectIp', 'Ping', 'WellknownIp',
    'DnsResolution', 'DnsOverHttps',
    'VpnConnectivity', 'VpnTunnel',
    'ServiceHealth'
  ];
  
  // Initialize form data when node changes or modal opens
  $: if (isOpen) {
    resetForm();
  }
  
  function resetForm() {
    if (node) {
      formData = {
        name: node.name,
        domain: node.domain || '',
        ip: node.ip || '',
        port: node.port?.toString() || '',
        path: node.path || '',
        description: node.description || '',
        node_type: node.node_type || '',
        capabilities: [...node.capabilities],
        monitoring_enabled: node.monitoring_enabled,
        assigned_tests: node.assigned_tests.map(t => t.test_type)
      };
    } else {
      formData = {
        name: '',
        domain: '',
        ip: '',
        port: '',
        path: '',
        description: '',
        node_type: '',
        capabilities: [],
        monitoring_enabled: false,
        assigned_tests: []
      };
    }
    errors = {};
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
    // Convert form data to proper types
    const nodeData = {
      name: formData.name.trim(),
      domain: formData.domain.trim() || undefined,
      ip: formData.ip.trim() || undefined,
      port: formData.port ? Number(formData.port) : undefined,
      path: formData.path.trim() || undefined,
      description: formData.description.trim() || undefined,
      node_type: formData.node_type || undefined,
      capabilities: formData.capabilities,
      monitoring_enabled: formData.monitoring_enabled,
      // Note: assigned_tests would need more complex handling for full test configs
      // This is simplified for the generic approach
    };
    
    if (!validateForm()) {
      return;
    }
    
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
  
  function getCapabilityDisplayName(capability: NodeCapability): string {
    return capability; // Could be enhanced with a mapping function
  }
  
  function getTestDisplayName(testType: string): string {
    return getTestTypeDisplayName(testType as TestType);
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
  submitLabel={isEditing ? 'Update Node' : 'Create Node'}
>
  <!-- Basic Information -->
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
        bind:value={formData.node_type}
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        <option value="">Select node type</option>
        {#each nodeTypes as type}
          <option value={type}>{getNodeTypeDisplayName(type)}</option>
        {/each}
      </select>
    </div>
  </div>
  
  <!-- Connection Information -->
  <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
    <div>
      <label for="ip" class="block text-sm font-medium text-gray-300 mb-1">
        IP Address
      </label>
      <input
        id="ip"
        name="ip"
        bind:value={formData.ip}
        type="text"
        placeholder="192.168.1.100"
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>
    
    <div>
      <label for="domain" class="block text-sm font-medium text-gray-300 mb-1">
        Domain/Hostname
      </label>
      <input
        id="domain"
        name="domain"
        bind:value={formData.domain}
        type="text"
        placeholder="server.local"
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>
    
    <div>
      <label for="port" class="block text-sm font-medium text-gray-300 mb-1">
        Port
      </label>
      <input
        id="port"
        name="port"
        bind:value={formData.port}
        type="number"
        min="1"
        max="65535"
        placeholder="80"
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        class:border-red-500={errors.port}
      />
      {#if errors.port}
        <p class="text-red-400 text-xs mt-1">{errors.port}</p>
      {/if}
    </div>
  </div>
  
  <!-- Path and Description -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <div>
      <label for="path" class="block text-sm font-medium text-gray-300 mb-1">
        Path
      </label>
      <input
        id="path"
        name="path"
        bind:value={formData.path}
        type="text"
        placeholder="/api"
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>
    
    <div>
      <label class="flex items-center space-x-2 pt-7">
        <input
          type="checkbox"
          name="monitoring_enabled"
          bind:checked={formData.monitoring_enabled}
          class="rounded bg-gray-700 border-gray-600 text-blue-600 focus:ring-blue-500"
        />
        <span class="text-sm font-medium text-gray-300">Enable Monitoring</span>
      </label>
    </div>
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
      placeholder="Optional description..."
      class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
    ></textarea>
  </div>
  
  <!-- Capabilities List Manager -->
  <ListManager
    label="Capabilities"
    bind:items={formData.capabilities}
    availableOptions={nodeCapabilities.map(cap => ({
      id: cap,
      label: getCapabilityDisplayName(cap)
    }))}
    placeholder="Select a capability to add"
    allowReorder={false}
    getDisplayName={(id: string) => getCapabilityDisplayName(id as NodeCapability)}
  />
  
  <!-- Tests List Manager (simplified - in real implementation you'd want full test config) -->
  <!-- <ListManager
    label="Assigned Tests"
    bind:items={formData.assigned_tests}
    availableOptions={testTypes.map(test => ({
      id: test,
      label: getTestDisplayName(test)
    }))}
    placeholder="Select a test to assign"
    allowReorder={false}
    getDisplayName={getTestDisplayName}
  /> -->
  
  <!-- Note about test assignment -->
  <div class="bg-blue-900/20 border border-blue-500/30 rounded-lg p-3">
    <p class="text-blue-300 text-sm">
      <strong>Note:</strong> Test assignments are managed separately through the "Assign Test" button on the node card, 
      as they require detailed configuration beyond just the test type.
    </p>
  </div>
</GenericEditModal>