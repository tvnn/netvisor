<!-- In src/lib/components/nodes/NodeEditModal/Capabilities/CapabilitiesForm.svelte -->
<script lang="ts">
  import { capabilities as allCapabilities, getCapabilityDescription, getCapabilityDisplay, getNodeTypeDisplay, getNodeTypeMetadata } from '$lib/components/shared/stores/registry';
	import RichRadioCheck from '$lib/components/shared/components/forms/RichRadioCheck.svelte';
  
  export let selectedCapabilities: Record<string, any>[];
  export let nodeType: string;
  
  let recommendations: string[] = [];
  
  // Extract capability types from the complex backend structure for UI
  $: selectedCapabilityTypes = selectedCapabilities.map(cap => {
    const keys = Object.keys(cap);
    return keys.length > 0 ? keys[0] : '';
  }).filter(type => type !== '');
  
  function handleCapabilitiesChange(value: string | string[]) {
    if (Array.isArray(value)) {
      // Convert selected capability type strings back to complex backend structure
      selectedCapabilities = value.map(capabilityType => {
        // Check if we already have this capability to preserve existing source data
        const existing = selectedCapabilities.find(cap => Object.keys(cap)[0] === capabilityType);
        if (existing) {
          return existing; // Preserve existing configuration and source data
        }
        
        // Create new capability with default source (manual entry)
        const newCapability: Record<string, any> = {
          [capabilityType]: {
            source: {
              port_detection: null,
              process_detection: null,
              manual: true,
              system: false
            }
          }
        };
        
        // Add daemon_id for DaemonService capabilities
        if (capabilityType === 'DaemonService') {
          newCapability[capabilityType].daemon_id = null;
        }
        
        return newCapability;
      });
    }
  }
  
  function applySuggested() {
    if (recommendations && recommendations.length > 0) {
      handleCapabilitiesChange(recommendations);
    }
  }

  $: if (nodeType) {
    const nodeTypeMetadata = $getNodeTypeMetadata(nodeType);
    recommendations = nodeTypeMetadata?.['typical_capabilities'] || [];
    
    // Only auto-apply on initial load when no capabilities are selected
    if (selectedCapabilities.length === 0 && recommendations.length > 0) {
      handleCapabilitiesChange(recommendations);
    }
  }
    
  // Transform capabilities into RichRadioCheck format and handle grouping
  $: capabilityOptions = $allCapabilities.map(capability => {
    const isSuggested = recommendations?.includes(capability.id) || false;
    return {
      id: capability.id,
      title: $getCapabilityDisplay(capability.id),
      description: $getCapabilityDescription(capability.id),
      category: capability.category,
      metadata: { ...capability.metadata, suggested: isSuggested }
    };
  });
  
  // Group capabilities by suggested vs other
  $: suggestedCapabilities = capabilityOptions.filter(cap => cap.metadata?.suggested);
  $: otherCapabilities = capabilityOptions.filter(cap => !cap.metadata?.suggested);
</script>

<div class="space-y-4">
  <!-- Header with description -->
  <p class="text-sm text-gray-400">
    Capabilities help determine which tests are compatible with this node and enable 
    automatic test recommendations. Select the services and access methods available on this device.
  </p>
  
  <!-- Reset to Suggested Button -->
  {#if recommendations && recommendations.length > 0}
    <div class="flex justify-between items-center">
      <div></div>
      <button
        type="button"
        on:click={applySuggested}
        class="text-sm text-blue-400 hover:text-blue-300 underline"
      >
        Reset to Suggested
      </button>
    </div>
  {/if}
  
  <!-- Capabilities Selection -->
  {#if recommendations !== undefined}
    <div class="space-y-6">
      <!-- Suggested Capabilities -->
      {#if suggestedCapabilities.length > 0}
        <div>
          <h4 class="text-sm font-medium mb-3 flex items-center gap-2">
            <span class="w-2 h-2 bg-blue-400 rounded-full"></span>
            <span class="text-blue-300">Suggested for {$getNodeTypeDisplay(nodeType)}</span>
          </h4>
          
          <RichRadioCheck
            mode="checkbox"
            name="capabilities"
            options={suggestedCapabilities}
            bind:selectedValues={selectedCapabilityTypes}
            onChange={handleCapabilitiesChange}
            columns={2}
          />
        </div>
      {/if}
      
      <!-- Other Capabilities -->
      {#if otherCapabilities.length > 0}
        <div>
          <h4 class="text-sm font-medium mb-3 flex items-center gap-2">
            <span class="w-2 h-2 bg-gray-500 rounded-full"></span>
            <span class="text-gray-300">Other Available Capabilities</span>
          </h4>
          
          <RichRadioCheck
            mode="checkbox"
            name="capabilities"
            options={otherCapabilities}
            bind:selectedValues={selectedCapabilityTypes}
            onChange={handleCapabilitiesChange}
            columns={2}
          />
        </div>
      {/if}
    </div>
  {:else}
    <!-- Initial loading state -->
    <div class="flex items-center justify-center py-8">
      <div class="flex items-center gap-3 text-gray-400">
        <div class="w-5 h-5 border-2 border-gray-400 border-t-transparent rounded-full animate-spin"></div>
        Loading capabilities...
      </div>
    </div>
  {/if}
  
  <!-- Summary -->
  <div class="pt-4 border-t border-gray-600">
    {#if selectedCapabilityTypes.length === 0}
      <p class="text-sm text-yellow-400">
        ⚠️ No capabilities selected. Consider selecting at least SSH Access for remote management.
      </p>
    {:else}
      <p class="text-sm text-gray-400">
        <span class="font-medium text-white">{selectedCapabilityTypes.length}</span> 
        capabilit{selectedCapabilityTypes.length === 1 ? 'y' : 'ies'} selected
        {#if recommendations.length > 0}
          • <span class="text-blue-400">{recommendations.filter(cap => selectedCapabilityTypes.includes(cap)).length} of {recommendations.length} suggested</span>
        {/if}
      </p>
    {/if}
  </div>
</div>