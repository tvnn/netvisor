<script lang="ts">
  import { Network } from 'lucide-svelte';
  import Tag from '../data/Tag.svelte';
  import type { Host } from '$lib/features/hosts/types/base';
  import type { TagProps } from '../data/types';
  import { getHostTargetString } from '$lib/features/hosts/store';
  import { serviceTypes } from '$lib/shared/stores/registry';
  
  export let host: Host;
  export let icon: any = Network;
  export let iconColor: string = 'text-blue-400';
  export let showTags: boolean = true;
  export let filterByService: keyof ReturnType<typeof serviceTypes.getMetadata> | null = null;
  
  // Get relevant service tags if filtering
  function getHostTags(host: Host): TagProps[] {
    if (!filterByService || !showTags) return [];
    
    const relevantServices = host.services.filter(service => 
      serviceTypes.getMetadata(service.service_type.type)[filterByService!]
    );
    
    return relevantServices.map(service => ({
      label: service.service_type.type,
      color: serviceTypes.getColorString(service.service_type.type)
    }));
  }
  
  $: hostTags = getHostTags(host);
</script>

<div class="flex items-center gap-3 flex-1 min-w-0">
  <!-- Icon -->
  <div class="w-6 h-6 rounded bg-gray-600/50 flex items-center justify-center flex-shrink-0">
    <svelte:component 
      this={icon} 
      class="w-3 h-3 {iconColor}" 
    />
  </div>
  
  <!-- Host info and description -->
  <div class="flex-1 min-w-0 text-left">
    <div class="flex gap-3 pb-1 items-center">
      <span class="block truncate font-medium text-white">{host.name}</span>
      
      <!-- Service tags -->
      {#if hostTags.length > 0}
        <div class="flex gap-1">
          {#each hostTags as tag}
            <Tag
              label={tag.label}
              color={tag.color}
              textColor={tag.textColor}
              bgColor={tag.bgColor} 
            />
          {/each}
        </div>
      {/if}
    </div>
    
    <!-- Target info -->
    <span class="block text-xs text-gray-400 truncate">
      {getHostTargetString(host.target)}
    </span>
    
    <!-- Description if available -->
    {#if host.description}
      <span class="block text-xs text-gray-500 truncate mt-1">
        {host.description}
      </span>
    {/if}
  </div>
</div>