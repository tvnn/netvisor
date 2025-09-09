<script lang="ts">
  import { hosts } from '$lib/features/hosts/store';
  import { getHostTargetString } from '$lib/features/hosts/store';
  import ListManager from '$lib/shared/components/forms/ListManager.svelte';
  import type { Host } from '$lib/features/hosts/types/base';
	import { services } from '$lib/shared/stores/registry';
	import type { Service } from '$lib/features/services/types/base';
  
  export let form: any;
  export let resolverIds: string[] = [];
  
  // Get hosts that can act as DNS resolvers (have DNS or AdBlock services)
  $: dnsCapableHosts = $hosts.filter((host: Host) => 
    host.services.some(service => services.getMetadata(service.type).can_be_dns_resolver)
  );
  
  // Get available hosts (not already selected)
  $: availableHosts = dnsCapableHosts.filter(host => 
    !resolverIds.includes(host.id)
  );
  
  // Convert resolver IDs to host objects for display
  $: selectedResolvers = resolverIds.map(id => 
    $hosts.find(host => host.id === id)
  ).filter(Boolean) as Host[];
  
  // Display functions for available hosts (dropdown options)
  function getOptionId(host: Host): string {
    return host.id;
  }
  
  function getOptionLabel(host: Host): string {
    return host.name;
  }
  
  function getOptionDescription(host: Host): string {
    return getHostTargetString(host.target);
  }
  
  function getOptionTags(host: Host) {
    const dnsServices: Service[] = host.services.filter(service => services.getMetadata(service.type).can_be_dns_resolver)
    
    return dnsServices.map(service => ({
      label: service.type,
      color: 'blue'
    }));
  }

  // Event handlers
  function handleAdd(hostId: string) {
    if (!resolverIds.includes(hostId)) {
      resolverIds = [...resolverIds, hostId];
    }
  }
  
  function handleRemove(index: number) {
    resolverIds = resolverIds.filter((_, i) => i !== index);
  }
</script>

<ListManager
  label="DNS Resolvers"
  helpText="Select hosts that provide DNS resolution services for this subnet"
  placeholder="Select a DNS server to add..."
  emptyMessage="No DNS resolvers configured. DNS capable hosts will appear here."
  allowReorder={true}
  
  options={availableHosts}
  items={selectedResolvers}
  allowItemEdit={(item) => false}
  
  {getOptionId}
  {getOptionLabel}
  {getOptionDescription}
  getOptionIcon={() => services.getIconComponent('DNS')}
  getOptionIconColor={() => 'text-blue-400'}
  {getOptionTags}
  
  getItemId={getOptionId}
  getItemLabel={getOptionLabel}
  getItemDescription={getOptionDescription}
  getItemIcon={() => services.getIconComponent('DNS')}
  getItemIconColor={() => 'text-green-400'}
  getItemTags={getOptionTags}
  
  onAdd={handleAdd}
  onRemove={handleRemove}
  onEdit={() => {}}
/>