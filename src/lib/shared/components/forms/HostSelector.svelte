<script lang="ts">
  import ListManager from './ListManager.svelte';
  import HostListItem from './HostListItem.svelte';
  import { hosts } from '$lib/features/hosts/store';
  import type { Host } from '$lib/features/hosts/types/base';
  import { serviceTypes } from '$lib/shared/stores/registry';
  import type { TagProps } from '../data/types';
  import { Network } from 'lucide-svelte';
  
  // Multi-select mode props (existing API)
  export let selectedIds: string[] = [];
  
  // Configuration props (existing API)
  export let label: string;
  export let helpText: string = '';
  export let placeholder: string = 'Select hosts';
  export let emptyMessage: string = 'No hosts available';
  export let serviceMetadataField: keyof ReturnType<typeof serviceTypes.getMetadata>; // e.g., 'can_be_dns_resolver', 'can_be_gateway', 'is_reverse_proxy'
  export let icon: any = Network;
  export let iconColor: string = 'text-blue-400';
  export let selectedIconColor: string = 'text-green-400';
  
  // Get hosts that match the specified criteria
  $: capableHosts = $hosts.filter((host: Host) => 
    host.services.some(service => serviceTypes.getMetadata(service.service_type.type)[serviceMetadataField])
  );
  
  // Get available hosts (not already selected)
  $: availableHosts = capableHosts.filter(host => 
    !selectedIds.includes(host.id)
  );
  
  // Convert selected IDs to host objects for display
  $: selectedHosts = selectedIds.map(id => 
    $hosts.find(host => host.id === id)
  ).filter(Boolean) as Host[];
  
  // Display functions - these are used by RichSelect internally for the dropdown
  function getOptionId(host: Host): string {
    return host.id;
  }
  
  function getOptionLabel(host: Host): string {
    return host.name;
  }
  
  function getOptionDescription(host: Host): string {
    if (host.target.type === 'IpAddress') {
      return host.target.config.ip;
    } else if (host.target.type === 'Hostname') {
      return host.target.config.hostname;
    }
    return '';
  }
  
  function getOptionTags(host: Host): TagProps[] {
    const relevantServices = host.services.filter(service => 
      serviceTypes.getMetadata(service.service_type.type)[serviceMetadataField]
    );
    
    return relevantServices.map(service => ({
      label: service.service_type.type,
      color: serviceTypes.getColorString(service.service_type.type)
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
>
  <!-- Only use the item slot for selected items display -->
  <svelte:fragment slot="item" let:item>
    <HostListItem 
      host={item} 
      {icon}
      iconColor={selectedIconColor}
      filterByService={serviceMetadataField}
    />
  </svelte:fragment>
</ListManager>