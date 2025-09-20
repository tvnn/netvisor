<script lang="ts">
  import { Network, Router, Search } from 'lucide-svelte';
  import { createEmptySubnetFormData } from '../store';
  import EditModal from '$lib/shared/components/forms/EditModal.svelte';
  import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
  import { serviceTypes, subnetTypes } from '$lib/shared/stores/registry';
  import { hosts } from '$lib/features/hosts/store';
  import type { Host } from '$lib/features/hosts/types/base';
	import { HostDisplay } from '$lib/shared/components/forms/selection/display/HostDisplay.svelte';
	import { services } from '$lib/features/services/store';
  
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

  let dnsServiceHostIds = $services.filter(service => serviceTypes.getMetadata(service.service_type.type)?.can_be_dns_resolver).map(service => service.host_id);
  let gatewayServiceHostIds = $services.filter(service => serviceTypes.getMetadata(service.service_type.type)?.can_be_dns_resolver).map(service => service.host_id);
  let reverseProxyServiceHostIds = $services.filter(service => serviceTypes.getMetadata(service.service_type.type)?.can_be_dns_resolver).map(service => service.host_id);
  
  // Filter hosts by service capabilities
  $: dnsCapableHosts = $hosts.filter(host => dnsServiceHostIds.includes(host.id));
  $: gatewayCapableHosts = $hosts.filter(host => gatewayServiceHostIds.includes(host.id));
  $: reverseProxyCapableHosts = $hosts.filter(host => reverseProxyServiceHostIds.includes(host.id));
  
  // Convert IDs to host objects for display
  $: selectedDnsResolvers = formData.dns_resolvers?.map(id => 
    $hosts.find(h => h.id === id)
  ).filter(Boolean) as Host[] || [];
  
  $: selectedGateways = formData.gateways?.map(id => 
    $hosts.find(h => h.id === id)
  ).filter(Boolean) as Host[] || [];
  
  $: selectedReverseProxies = formData.reverse_proxies?.map(id => 
    $hosts.find(h => h.id === id)
  ).filter(Boolean) as Host[] || [];
  
  // Available hosts (filtered out already selected)
  $: availableDnsHosts = dnsCapableHosts.filter(host => 
    !formData.dns_resolvers?.includes(host.id)
  );
  
  $: availableGatewayHosts = gatewayCapableHosts.filter(host => 
    !formData.gateways?.includes(host.id)
  );
  
  $: availableReverseProxyHosts = reverseProxyCapableHosts.filter(host => 
    !formData.reverse_proxies?.includes(host.id)
  );
  
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
  
  // Event handlers for DNS resolvers
  function handleAddDnsResolver(hostId: string) {
    if (!formData.dns_resolvers?.includes(hostId)) {
      formData.dns_resolvers = [...(formData.dns_resolvers || []), hostId];
    }
  }
  
  function handleRemoveDnsResolver(index: number) {
    formData.dns_resolvers = formData.dns_resolvers?.filter((_, i) => i !== index) || [];
  }
  
  // Event handlers for gateways
  function handleAddGateway(hostId: string) {
    if (!formData.gateways?.includes(hostId)) {
      formData.gateways = [...(formData.gateways || []), hostId];
    }
  }
  
  function handleRemoveGateway(index: number) {
    formData.gateways = formData.gateways?.filter((_, i) => i !== index) || [];
  }
  
  // Event handlers for reverse proxies
  function handleAddReverseProxy(hostId: string) {
    if (!formData.reverse_proxies?.includes(hostId)) {
      formData.reverse_proxies = [...(formData.reverse_proxies || []), hostId];
    }
  }
  
  function handleRemoveReverseProxy(index: number) {
    formData.reverse_proxies = formData.reverse_proxies?.filter((_, i) => i !== index) || [];
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
              {#each subnetTypes.getItems() as subnet_type}
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
              <ListManager
                label="DNS Resolvers"
                helpText="Select hosts that provide DNS resolution services for this subnet"
                placeholder="Select a DNS server to add..."
                emptyMessage="No DNS resolvers configured. DNS capable hosts will appear here."
                allowReorder={false}
                
                options={availableDnsHosts}
                items={selectedDnsResolvers}
                allowItemEdit={() => false}
                
                optionDisplayComponent={HostDisplay}
                itemDisplayComponent={HostDisplay}
                
                onAdd={handleAddDnsResolver}
                onRemove={handleRemoveDnsResolver}
                onEdit={() => {}}
              />
            </div>
          </div>
        </div>

        <!-- Gateways Section -->
        <div class="space-y-4">
          <div class="border-t border-gray-700 pt-6">
            <h3 class="text-lg font-medium text-white mb-4">Gateways</h3>
            <div class="bg-gray-800/50 rounded-lg p-4">
              <ListManager
                label="Gateways"
                helpText="Select hosts that provide gateway/routing services for this subnet"
                placeholder="Select a gateway to add..."
                emptyMessage="No gateways configured. Gateway-capable hosts will appear here."
                allowReorder={false}
                
                options={availableGatewayHosts}
                items={selectedGateways}
                allowItemEdit={() => false}
                
                optionDisplayComponent={HostDisplay}
                itemDisplayComponent={HostDisplay}
                
                onAdd={handleAddGateway}
                onRemove={handleRemoveGateway}
                onEdit={() => {}}
              />
            </div>
          </div>
        </div>

        <!-- Reverse Proxies Section -->
        <div class="space-y-4">
          <div class="border-t border-gray-700 pt-6">
            <h3 class="text-lg font-medium text-white mb-4">Reverse Proxies</h3>
            <div class="bg-gray-800/50 rounded-lg p-4">
              <ListManager
                label="Reverse Proxies"
                helpText="Select hosts that provide reverse proxy services for this subnet"
                placeholder="Select a reverse proxy to add..."
                emptyMessage="No reverse proxies configured. Reverse proxy-capable hosts will appear here."
                allowReorder={false}
                
                options={availableReverseProxyHosts}
                items={selectedReverseProxies}
                allowItemEdit={() => false}
                
                optionDisplayComponent={HostDisplay}
                itemDisplayComponent={HostDisplay}
                
                onAdd={handleAddReverseProxy}
                onRemove={handleRemoveReverseProxy}
                onEdit={() => {}}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</EditModal>