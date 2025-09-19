<script lang="ts">
  import { Network } from 'lucide-svelte';
  import RichSelect from './RichSelect.svelte';
  import HostListItem from './HostListItem.svelte';
  import { hosts } from '$lib/features/hosts/store';
  import type { Host } from '$lib/features/hosts/types/base';
  import { serviceTypes } from '$lib/shared/stores/registry';
  import type { TagProps } from '../data/types';
  
  // Selection props
  export let selectedId: string = '';
  export let onSelect: ((hostId: string) => void) | null = null;
  
  // Configuration props
  export let label: string = '';
  export let placeholder: string = 'Select a host';
  export let error: string = '';
  export let excludeIds: string[] = []; // Hosts to exclude from selection
  export let filterByService: keyof ReturnType<typeof serviceTypes.getMetadata> | null = null;
  export let icon: any = Network;
  export let iconColor: string = 'text-blue-400';
  export let disabled: boolean = false;
  
  // Get hosts that match the specified criteria
  $: capableHosts = filterByService 
    ? $hosts.filter((host: Host) => 
        host.services.some(service => serviceTypes.getMetadata(service.service_type.type)[filterByService!])
      )
    : $hosts;
  
  // Get available hosts (not excluded)
  $: availableHosts = capableHosts.filter(host => 
    !excludeIds.includes(host.id)
  );
  
  // Display functions for RichSelect
  function getOptionId(host: Host): string {
    return host.id;
  }
  
  function getOptionLabel(host: Host): string {
    return host.name;
  }
  
  function getOptionDescription(host: Host): string {
    const parts = [];
    
    // Add target info
    if (host.target.type === 'IpAddress') {
      parts.push(host.target.config.ip);
    } else if (host.target.type === 'Hostname') {
      parts.push(host.target.config.hostname);
    }
    
    // Add description if available
    if (host.description) {
      parts.push(host.description);
    }
    
    return parts.join(' • ');
  }
  
  function getOptionTags(host: Host): TagProps[] {
    if (!filterByService) return [];
    
    const relevantServices = host.services.filter(service => 
      serviceTypes.getMetadata(service.service_type.type)[filterByService!]
    );
    
    return relevantServices.map(service => ({
      label: service.service_type.type,
      color: serviceTypes.getColorString(service.service_type.type)
    }));
  }
  
  function handleSelect(hostId: string) {
    selectedId = hostId;
    onSelect?.(hostId);
  }
</script>

<RichSelect
  {label}
  {placeholder}
  {error}
  {disabled}
  selectedValue={selectedId}
  options={availableHosts}
  onSelect={handleSelect}
  
  {getOptionId}
  {getOptionLabel}
  {getOptionDescription}
  getOptionIcon={() => icon}
  getOptionIconColor={() => iconColor}
  {getOptionTags}
  getOptionIsDisabled={() => false}
  getOptionCategory={() => null}
/>