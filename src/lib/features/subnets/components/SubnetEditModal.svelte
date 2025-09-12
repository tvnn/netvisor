<script lang="ts">
  import { Network, Router } from 'lucide-svelte';
  import { createEmptySubnetFormData } from '../store';
	import EditModal from '$lib/shared/components/forms/EditModal.svelte';
	import { services, subnet_types } from '$lib/shared/stores/registry';
	import { get } from 'svelte/store';
	import HostSelector from '$lib/shared/components/forms/HostSelector.svelte';
  
  export let subnet: Subnet | null = null;
  export let isOpen = false;
  export let onCreate: (data: Subnet) => Promise<void> | void;
  export let onUpdate: (id: string, data: Subnet) => Promise<void> | void;
  export let onClose: () => void;
  export let onDelete: ((id: string) => Promise<void> | void) | null = null;
  
  let loading = false;
  let deleting = false;
  
  $: isEditing = subnet !== null;
  $: title = isEditing ? `Edit ${subnet?.name}` : 'Create Subnet';
  
  let formData: Subnet = createEmptySubnetFormData();
  
  // Initialize form data when subnet changes or modal opens
  $: if (isOpen) {
    resetForm();
  }
  
  function resetForm() {
    formData = subnet ? { ...subnet } : createEmptySubnetFormData();
  }
  
  async function handleSubmit() {
    // Clean up the data before sending
    const subnetData: Subnet = {
      ...formData,
      name: formData.name.trim(),
      description: formData.description?.trim() || '',
      cidr: formData.cidr.trim(),
    };
    
    loading = true;
    try {
      if (isEditing && subnet) {
        await onUpdate(subnet.id, subnetData);
      } else {
        await onCreate(subnetData);
      }
    } finally {
      loading = false;
    }
  }
  
  async function handleDelete() {
    if (onDelete && subnet) {
      deleting = true;
      try {
        await onDelete(subnet.id);
      } finally {
        deleting = false;
      }
    }
  }
  
  // Dynamic labels based on create/edit mode
  $: saveLabel = isEditing ? 'Update Subnet' : 'Create Subnet';
  $: cancelLabel = 'Cancel';
</script>

<EditModal
  {isOpen}
  {title}
  {loading}
  {deleting}
  {saveLabel}
  {cancelLabel}
  onSave={handleSubmit}
  onCancel={onClose}
  onDelete={isEditing ? handleDelete : null}
  size="xl"
  let:form
>
  <!-- Header icon -->
  <svelte:fragment slot="header-icon">
    <div class="p-2 bg-orange-600/20 rounded-lg">
      <Network class="w-5 h-5 text-orange-400" />
    </div>
  </svelte:fragment>
  
  <!-- Content -->
  <div class="h-full flex flex-col overflow-hidden">
    <div class="flex-1 overflow-y-auto">
      <div class="space-y-8 p-6">
        <!-- Basic Information -->
        <div class="space-y-4">
          <h3 class="text-lg font-medium text-white">Subnet Details</h3>
          
          <!-- Name -->
          <div>
            <label for="name" class="block text-sm font-medium text-gray-300 mb-2">
              Name <span class="text-red-400">*</span>
            </label>
            <input
              id="name"
              type="text"
              bind:value={formData.name}
              placeholder="e.g., Home LAN, VPN Network"
              required
              class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-transparent"
            />
          </div>
          
          <!-- CIDR -->
          <div>
            <label for="cidr" class="block text-sm font-medium text-gray-300 mb-2">
              CIDR <span class="text-red-400">*</span>
            </label>
            <input
              id="cidr"
              type="text"
              bind:value={formData.cidr}
              placeholder="e.g., 192.168.1.0/24, 10.0.0.0/8"
              required
              class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-transparent"
            />
            <p class="mt-1 text-sm text-gray-400">
              Network address and prefix length (e.g., 192.168.1.0/24)
            </p>
          </div>
          
          <!-- Subnet Type -->
          <div>
            <label for="subnet_type" class="block text-sm font-medium text-gray-300 mb-2">
              Network Type
            </label>
            <select
              id="subnet_type"
              bind:value={formData.subnet_type}
              class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-transparent"
            >
              {#each subnet_types.getItems() as subnet_type}
                <option value="{subnet_type.id}">{subnet_type.display_name}</option>
              {/each}
            </select>
          </div>
          
          <!-- Description -->
          <div>
            <label for="description" class="block text-sm font-medium text-gray-300 mb-2">
              Description
            </label>
            <textarea
              id="description"
              bind:value={formData.description}
              placeholder="Optional description of this subnet..."
              rows="3"
              class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-transparent resize-none"
            ></textarea>
          </div>
        </div>

        <!-- DNS Resolvers Section -->
        <div class="space-y-4">
          <div class="border-t border-gray-700 pt-6">
            <h3 class="text-lg font-medium text-white mb-4">DNS Resolvers</h3>
            <div class="bg-gray-800/50 rounded-lg p-4">
              <HostSelector
                bind:selectedIds={formData.dns_resolvers}
                label="DNS Resolvers"
                helpText="Select hosts that provide DNS resolution services for this subnet"
                placeholder="Select a DNS server to add..."
                emptyMessage="No DNS resolvers configured. DNS capable hosts will appear here."
                serviceMetadataField="can_be_dns_resolver"
                icon={services.getIconComponent('DNS')}
                iconColor="text-blue-400"
                selectedIconColor="text-green-400"
              />
            </div>
          </div>
        </div>

        <!-- Gateways Section -->
        <div class="space-y-4">
          <div class="border-t border-gray-700 pt-6">
            <h3 class="text-lg font-medium text-white mb-4">Gateways</h3>
            <div class="bg-gray-800/50 rounded-lg p-4">
              <HostSelector
                bind:selectedIds={formData.gateways}
                label="Gateways"
                helpText="Select hosts that provide gateway/routing services for this subnet"
                placeholder="Select a gateway to add..."
                emptyMessage="No gateways configured. Gateway-capable hosts will appear here."
                serviceMetadataField="can_be_gateway"
                icon={Router}
                iconColor="text-orange-400"
                selectedIconColor="text-green-400"
              />
            </div>
          </div>
        </div>

        <!-- Reverse Proxies Section -->
        <div class="space-y-4">
          <div class="border-t border-gray-700 pt-6">
            <h3 class="text-lg font-medium text-white mb-4">Reverse Proxies</h3>
            <div class="bg-gray-800/50 rounded-lg p-4">
              <HostSelector
                bind:selectedIds={formData.reverse_proxies}
                label="Reverse Proxies"
                helpText="Select hosts that provide reverse proxy services for this subnet"
                placeholder="Select a reverse proxy to add..."
                emptyMessage="No reverse proxies configured. Reverse proxy-capable hosts will appear here."
                serviceMetadataField="is_reverse_proxy"
                icon={Router}
                iconColor="text-orange-400"
                selectedIconColor="text-green-400"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</EditModal>