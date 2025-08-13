<!-- src/lib/components/modals/NodeEditor.svelte -->
<script lang="ts">
  import { X } from 'lucide-svelte';
  import type { Node, NodeType, NodeCapability } from '../../types/nodes';
  import { getNodeTypeDisplayName } from '../../types/nodes';
  
  export let node: Node | null = null;
  export let isOpen = false;
  export let onCreate: (data: any) => void = () => {};
  export let onUpdate: (id: string, data: any) => void = () => {};
  export let onClose: () => void = () => {};
  
  let formData = {
    name: '',
    domain: '',
    ip: '',
    port: '',
    path: '',
    description: '',
    node_type: undefined as NodeType | undefined,
    capabilities: [] as NodeCapability[],
    monitoring_enabled: false
  };
  
  let loading = false;
  
  // Available options
  const nodeTypes: NodeType[] = [
    'Router', 'Switch', 'AccessPoint', 'Firewall',
    'WebServer', 'DatabaseServer', 'MediaServer', 'DnsServer', 'VpnServer', 'NasDevice',
    'Workstation', 'IotDevice', 'Printer', 'Camera',
    'UnknownDevice'
  ];
  
  const nodeCapabilities: NodeCapability[] = [
    'SshAccess', 'RdpAccess', 'VncAccess',
    'HttpService', 'HttpsService',
    'DatabaseService',
    'DnsService', 'EmailService', 'FtpService'
  ];
  
  // Reset form when node changes or modal opens
  $: if (isOpen) {
    resetForm();
  }
  
  function resetForm() {
    if (node) {
      // Editing existing node
      formData = {
        name: node.name,
        domain: node.domain || '',
        ip: node.ip || '',
        port: node.port?.toString() || '',
        path: node.path || '',
        description: node.description || '',
        node_type: node.node_type,
        capabilities: [...(node.capabilities || [])],
        monitoring_enabled: node.monitoring_enabled
      };
    } else {
      // Creating new node
      formData = {
        name: '',
        domain: '',
        ip: '',
        port: '',
        path: '',
        description: '',
        node_type: undefined,
        capabilities: [],
        monitoring_enabled: false
      };
    }
  }
  
  function handleCapabilityToggle(capability: NodeCapability) {
    if (formData.capabilities.includes(capability)) {
      formData.capabilities = formData.capabilities.filter(c => c !== capability);
    } else {
      formData.capabilities = [...formData.capabilities, capability];
    }
  }
  
  function handleNodeTypeChange() {
    // Auto-suggest capabilities based on node type
    if (formData.node_type) {
      // This would use the suggested_capabilities() method from the backend
      // For now, just keep existing capabilities
    }
  }
  
  async function handleSubmit() {
    loading = true;
    
    try {
      const submitData = {
        name: formData.name,
        domain: formData.domain || undefined,
        ip: formData.ip || undefined,
        port: formData.port ? parseInt(formData.port) : undefined,
        path: formData.path || undefined,
        description: formData.description || undefined,
        node_type: formData.node_type,
        capabilities: formData.capabilities,
        monitoring_enabled: formData.monitoring_enabled
      };
      
      if (node) {
        // Update existing node
        onUpdate(node.id, submitData);
      } else {
        // Create new node
        onCreate(submitData);
      }
    } catch (error) {
      console.error('Error saving node:', error);
    } finally {
      loading = false;
    }
  }
  
  function handleClose() {
    resetForm();
    onClose();
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-gray-800 rounded-lg p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-xl font-semibold text-white">
          {node ? 'Edit Node' : 'Create Node'}
        </h2>
        <button
          on:click={handleClose}
          class="text-gray-400 hover:text-white"
        >
          <X size={24} />
        </button>
      </div>
      
      <form on:submit|preventDefault={handleSubmit} class="space-y-4">
        <!-- Basic Information -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <!-- Name -->
          <div>
            <label for="name" class="block text-sm font-medium text-gray-300 mb-1">
              Name *
            </label>
            <input
              id="name"
              bind:value={formData.name}
              type="text"
              required
              class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Enter node name"
            />
          </div>
          
          <!-- Node Type -->
          <div>
            <label for="node_type" class="block text-sm font-medium text-gray-300 mb-1">
              Node Type
            </label>
            <select
              id="node_type"
              bind:value={formData.node_type}
              on:change={handleNodeTypeChange}
              class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value={undefined}>Select type...</option>
              {#each nodeTypes as nodeType}
                <option value={nodeType}>{getNodeTypeDisplayName(nodeType)}</option>
              {/each}
            </select>
          </div>
        </div>
        
        <!-- Network Information -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <!-- IP Address -->
          <div>
            <label for="ip" class="block text-sm font-medium text-gray-300 mb-1">
              IP Address
            </label>
            <input
              id="ip"
              bind:value={formData.ip}
              type="text"
              class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="192.168.1.100"
            />
          </div>
          
          <!-- Domain -->
          <div>
            <label for="domain" class="block text-sm font-medium text-gray-300 mb-1">
              Domain
            </label>
            <input
              id="domain"
              bind:value={formData.domain}
              type="text"
              class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="example.com"
            />
          </div>
        </div>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <!-- Port -->
          <div>
            <label for="port" class="block text-sm font-medium text-gray-300 mb-1">
              Port
            </label>
            <input
              id="port"
              bind:value={formData.port}
              type="number"
              min="1"
              max="65535"
              class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="80"
            />
          </div>
          
          <!-- Path -->
          <div>
            <label for="path" class="block text-sm font-medium text-gray-300 mb-1">
              Path
            </label>
            <input
              id="path"
              bind:value={formData.path}
              type="text"
              class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="/api"
            />
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
            placeholder="Optional description"
          ></textarea>
        </div>
        
        <!-- Capabilities -->
        <div>
          <label for="capabilities" class="block text-sm font-medium text-gray-300 mb-2">
            Capabilities
          </label>
          <div class="grid grid-cols-2 md:grid-cols-3 gap-2">
            {#each nodeCapabilities as capability}
              <label class="flex items-center space-x-2 text-sm">
                <input
                  type="checkbox"
                  checked={formData.capabilities.includes(capability)}
                  on:change={() => handleCapabilityToggle(capability)}
                  class="rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-blue-500"
                />
                <span class="text-gray-300">{capability}</span>
              </label>
            {/each}
          </div>
        </div>
        
        <!-- Monitoring -->
        <div>
          <label class="flex items-center space-x-2">
            <input
              type="checkbox"
              bind:checked={formData.monitoring_enabled}
              class="rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-blue-500"
            />
            <span class="text-sm text-gray-300">Enable monitoring</span>
          </label>
        </div>
        
        <!-- Action Buttons -->
        <div class="flex justify-end space-x-3 pt-4">
          <button
            type="button"
            on:click={handleClose}
            class="px-4 py-2 text-gray-300 hover:text-white border border-gray-600 rounded-md hover:border-gray-500 transition-colors"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={loading || !formData.name}
            class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed transition-colors"
          >
            {loading ? 'Saving...' : (node ? 'Update Node' : 'Create Node')}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  input[type="checkbox"] {
    accent-color: #3b82f6;
  }
</style>