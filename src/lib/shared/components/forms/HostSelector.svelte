<script lang="ts">
  import { hosts } from '$lib/features/hosts/store';
  import { getHostTargetString } from '$lib/features/hosts/store';
  import ListManager from '$lib/shared/components/forms/ListManager.svelte';
  import type { Host } from '$lib/features/hosts/types/base';
  import { services } from '$lib/shared/stores/registry';
  import type { TagProps } from '$lib/shared/components/data/types';
  
  export let selectedIds: string[] = [];
  
  // Configuration props
  export let label: string;
  export let helpText: string;
  export let placeholder: string;
  export let emptyMessage: string;
  export let serviceMetadataField: keyof ReturnType<typeof services.getMetadata>; // e.g., 'can_be_dns_resolver', 'can_be_gateway', 'is_reverse_proxy'
  export let icon: any;
  export let iconColor: string = 'text-blue-400';
  export let selectedIconColor: string = 'text-green-400';
  
  // Get hosts that match the specified criteria
  $: capableHosts = $hosts.filter((host: Host) => 
    host.services.some(service => services.getMetadata(service.type)[serviceMetadataField])
  );
  
  // Get available hosts (not already selected)
  $: availableHosts = capableHosts.filter(host => 
    !selectedIds.includes(host.id)
  );
  
  // Convert selected IDs to host objects for display
  $: selectedHosts = selectedIds.map(id => 
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
  
  function getOptionTags(host: Host): TagProps[] {
    const relevantServices = host.services.filter(service => 
      services.getMetadata(service.type)[serviceMetadataField]
    );
    
    return relevantServices.map(service => ({
      label: service.type,
      color: services.getColorString(service.type)
    }));
  }
  
  // Event handlers
  function handleAdd(hostId: string) {
    if (!selectedIds.includes(hostId)) {
      selectedIds = [...selectedIds, hostId];
    }
  }
  
  function handleRemove(index: number) {
    selectedIds = selectedIds.filter((_, i) => i !== index);
  }
</script>

<ListManager
  {label}
  {helpText}
  {placeholder}
  {emptyMessage}
  allowReorder={true}
  
  options={availableHosts}
  items={selectedHosts}
  allowItemEdit={(item) => false}
  
  {getOptionId}
  {getOptionLabel}
  {getOptionDescription}
  getOptionIcon={() => icon}
  getOptionIconColor={() => iconColor}
  {getOptionTags}
  
  getItemId={getOptionId}
  getItemLabel={getOptionLabel}
  getItemDescription={getOptionDescription}
  getItemIcon={() => icon}
  getItemIconColor={() => selectedIconColor}
  getItemTags={getOptionTags}
  
  onAdd={handleAdd}
  onRemove={handleRemove}
  onEdit={() => {}}
/>