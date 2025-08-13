<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { X } from 'lucide-svelte';
  import type { Node, NodeType, NodeCapability } from '../../types';
  import { getNodeTypeDisplayName } from '../../types';
  
  export let node: Node | null = null;
  export let isOpen = false;
  
  const dispatch = createEventDispatcher();
  
  let formData = {
    name: '',
    domain: '',
    ip: '',
    port: '',
    path: '',
    description: '',
    node_type: 'UnknownDevice' as NodeType,
    capabilities: [] as NodeCapability[],
    monitoring_enabled: false
  };
  
  let loading = false;
  let errors: Record<string, string> = {};
  
  const nodeTypes: NodeType[] = [
    'Router', 'Switch', 'AccessPoint', 'Firewall',
    'WebServer', 'DatabaseServer', 'MediaServer', 
    'DnsServer', 'VpnServer', 'NasDevice',
    'Workstation', 'IotDevice', 'Printer', 'Camera',
    'UnknownDevice'
  ];
  
  const commonCapabilities: NodeCapability[] = [
    'SshAccess', 'RdpAccess', 'VncAccess',
    'HttpService', 'HttpsService',
    'MysqlService', 'PostgresService', 'MongoService', 'RedisService',
    'DnsService', 'EmailService', 'FtpService'
  ];
  
  $: isEditing = node !== null;
  $: title = isEditing ? `Edit ${node?.name}` : 'Create New Node';
  
  // Initialize form data when node changes
  $: if (node) {
    formData = {
      name: node.name,
      domain: node.domain || '',
      ip: node.ip || '',
      port: node.port?.toString() || '',
      path: node.path || '',
      description: node.description || '',
      node_type: node.node_type || 'UnknownDevice',
      capabilities: [...node.capabilities],
      monitoring_enabled: node.monitoring_enabled
    };
  } else {
    resetForm();
  }
  
  function resetForm() {
    formData = {
      name: '',
      domain: '',
      ip: '',
      port: '',
      path: '',
      description: '',
      node_type: 'UnknownDevice',
      capabilities: [],
      monitoring_enabled: false
    };
    errors = {};
  }
  
  function validateForm(): boolean {
    errors = {};
    
    if (!formData.name.trim()) {
      errors.name = 'Name is required';
    }
    
    if (formData.ip && !isValidIP(formData.ip)) {
      errors.ip = 'Invalid IP address format';
    }
    
    if (formData.port && !isValidPort(formData.port)) {
      errors.port = 'Port must be between 1 and 65535';
    }
    
    return Object.keys(errors).length === 0;
  }
  
  function isValidIP(ip: string): boolean {
    const ipRegex = /^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;
    return ipRegex.test(ip);
  }
  
  function isValidPort(port: string): boolean {
    const portNum = parseInt(port);
    return !isNaN(portNum) && portNum >= 1 && portNum <= 65535;
  }
  
  function toggleCapability(capability: NodeCapability) {
    if (formData.capabilities.includes(capability)) {
      formData.capabilities = formData.capabilities.filter(c => c !== capability);
    } else {
      formData.capabilities = [...formData.capabilities, capability];
    }
  }
  
  async function handleSubmit() {
    if (!validateForm()) return;
    
    loading = true;
    
    try {
      const requestData = {
        name: formData.name.trim(),
        domain: formData.domain.trim() || undefined,
        ip: formData.ip.trim() || undefined,
        port: formData.port ? parseInt(formData.port) : undefined,
        path: formData.path.trim() || undefined,
        description: formData.description.trim() || undefined,
        node_type: formData.node_type,
        capabilities: formData.capabilities,
        monitoring_enabled: formData.monitoring_enabled
      };
      
      if (isEditing && node) {
        dispatch('update', { id: node.id, data: requestData });
      } else {
        dispatch('create', requestData);
      }
      
      handleClose();
    } catch (error) {
      console.error('Form submission error:', error);
    } finally {
      loading = false;
    }
  }
  
  function handleClose() {
    dispatch('close');
    resetForm();
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
              Name *
            </label>
            <input
              id="name"
              bind:value={formData.name}
              type="text"
              class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              class:border-red-500={errors.name}
              placeholder="Enter node name"
              required
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
              bind:value={formData.node_type}
              class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              {#each nodeTypes as type}
                <option value={type}>{getNodeTypeDisplayName(type)}</option>
              {/each}
            </select>
          </div>
        </div>
        
        <!-- Network Information -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div>
            <label for="ip" class="block text-sm font-medium text-gray-300 mb-1">
              IP Address
            </label>
            <input
              id="ip"
              bind:value={formData.ip}
              type="text"
              class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              class:border-red-500={errors.ip}
              placeholder="192.168.1.100"
            />
            {#if errors.ip}
              <p class="text-red-400 text-xs mt-1">{errors.ip}</p>
            {/if}
          </div>
          
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
              class:border-red-500={errors.port}
              placeholder="80"
            />
            {#if errors.port}
              <p class="text-red-400 text-xs mt-1">{errors.port}</p>
            {/if}
          </div>
          
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
        
        <!-- Additional Fields -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label for="path" class="block text-sm font-medium text-gray-300 mb-1">
              Path
            </label>
            <input
              id="path"
              bind:value={formData.path}
              type="text"
              class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="/api/health"
            />
          </div>
          
          <div class="flex items-center">
            <input
              id="monitoring_enabled"
              bind:checked={formData.monitoring_enabled}
              type="checkbox"
              class="w-4 h-4 text-blue-600 bg-gray-700 border-gray-600 rounded focus:ring-blue-500"
            />
            <label for="monitoring_enabled" class="ml-2 text-sm text-gray-300">
              Enable monitoring
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
            placeholder="Optional description for this node"
          ></textarea>
        </div>
        
        <!-- Capabilities -->
        <div>
        <fieldset>
            <legend class="block text-sm font-medium text-gray-300 mb-2">
            Capabilities
            </legend>
            
            <div class="grid grid-cols-2 md:grid-cols-3 gap-2">
            {#each commonCapabilities as capability}
                <label class="flex items-center space-x-2 text-sm">
                <input
                    type="checkbox"
                    checked={formData.capabilities.includes(capability)}
                    on:change={() => toggleCapability(capability)}
                    class="w-4 h-4 text-blue-600 bg-gray-700 border-gray-600 rounded focus:ring-blue-500"
                />
                <span class="text-gray-300">{capability}</span>
                </label>
            {/each}
            </div>
        </fieldset>
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
              {isEditing ? 'Update' : 'Create'} Node
            {/if}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}