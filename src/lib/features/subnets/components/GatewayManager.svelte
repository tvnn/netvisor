<script lang="ts">
  import { Router } from 'lucide-svelte';
  import { hosts } from '$lib/features/hosts/store';
  import { getHostTargetString } from '$lib/features/hosts/store';
  import ListManager from '$lib/shared/components/forms/ListManager.svelte';
  import type { Host } from '$lib/features/hosts/types/base';
	import { services } from '$lib/shared/stores/registry';
	import type { TagProps } from '$lib/shared/components/data/types';
  
  export let form: any;
  export let gatewayIds: string[] = [];
  
  // Get hosts that can act as gateways (have Router services or gateway-like IPs)
  $: gatewayCapableHosts = $hosts.filter((host: Host) => 
    host.services.some(service => services.getMetadata(service.type).can_be_gateway));
  
  // Get available hosts (not already selected)
  $: availableHosts = gatewayCapableHosts.filter(host => 
    !gatewayIds.includes(host.id)
  );
  
  // Convert gateway IDs to host objects for display
  $: selectedGateways = gatewayIds.map(id => 
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
    return getHostTargetString(host.target)
  }
  
  function getOptionTags(host: Host) {
    const tags: TagProps[] = [];
    
    const gatewayServices = host.services.filter(service => services.getMetadata(service.type).can_be_gateway);
    
    gatewayServices.forEach(service => {
      tags.push({
        label: service.type,
        color: 'blue'
      });
    });
    
    return tags;
  }
  
  // Event handlers
  function handleAdd(hostId: string) {
    if (!gatewayIds.includes(hostId)) {
      gatewayIds = [...gatewayIds, hostId];
    }
  }
  
  function handleRemove(index: number) {
    gatewayIds = gatewayIds.filter((_, i) => i !== index);
  }
</script>

<ListManager
  label="Gateways"
  helpText="Select hosts that provide gateway/routing services for this subnet"
  placeholder="Select a gateway to add..."
  emptyMessage="No gateways configured. Gateway-capable hosts will appear here."
  allowReorder={true}
  
  options={availableHosts}
  items={selectedGateways}
  allowItemEdit={(item) => false}
  
  {getOptionId}
  {getOptionLabel}
  {getOptionDescription}
  getOptionIcon={() => Router}
  getOptionIconColor={() => 'text-orange-400'}
  {getOptionTags}
  
  getItemId={getOptionId}
  getItemLabel={getOptionLabel}
  getItemDescription={getOptionDescription}
  getItemIcon={() => Router}
  getItemIconColor={() => 'text-green-400'}
  getItemTags={getOptionTags}
  
  onAdd={handleAdd}
  onRemove={handleRemove}
  onEdit={() => {}}
/>