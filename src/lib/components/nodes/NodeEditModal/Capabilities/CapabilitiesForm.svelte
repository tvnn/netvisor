<script lang="ts">
  import { capabilities as allCapabilities, getCapabilityDescription, getCapabilityDisplay, getNodeTypeDisplay, getNodeTypeMetadata } from '$lib/api/registry';
  
  export let selectedCapabilities: string[];
  export let nodeType: string;
  
  let recommendations: string[];
  
  function handleCapabilityToggle(capability: string) {
    if (selectedCapabilities.includes(capability)) {
      selectedCapabilities = selectedCapabilities.filter(c => c !== capability);
    } else {
      selectedCapabilities = [...selectedCapabilities, capability];
    }
  }
  
  function applySuggested() {
    if (recommendations && recommendations.length > 0) {
      selectedCapabilities = [...recommendations];
    }
  }

  $: if (nodeType) {
    recommendations = $getNodeTypeMetadata(nodeType)['typical_capabilities'];
    selectedCapabilities = [...recommendations];
  }
    
  // Computed values
  $: suggestedCapabilities = recommendations || [];
  $: otherCapabilities = recommendations 
    ? $allCapabilities.filter(cap => !suggestedCapabilities.includes(cap.id))
    : [];
</script>

<div class="space-y-4">
  <!-- Header with description -->
  <p class="text-sm text-gray-400">
    Capabilities help determine which tests are compatible with this node and enable 
    automatic test recommendations. Select the services and access methods available on this device.
  </p>
  
  <!-- Capabilities List -->
  {#if recommendations}
    <div class="space-y-4">
      <!-- Suggested capabilities first (if any) -->
      {#if suggestedCapabilities.length > 0}
        <div>
          <div class="flex justify-between">
            <h4 class="text-sm font-medium text-blue-300 mb-3 flex items-center gap-2">
              <span class="w-2 h-2 bg-blue-400 rounded-full"></span>
              Suggested for {$getNodeTypeDisplay(nodeType)}
            </h4>
            {#if suggestedCapabilities.length > 0}
              <button
                type="button"
                on:click={applySuggested}
                class="text-sm text-blue-400 hover:text-blue-300 underline"
              >
                Reset to Suggested
              </button>
            {/if}
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
            {#each suggestedCapabilities as capability}
              <label class="flex items-start space-x-3 cursor-pointer p-3 bg-blue-900/10 border border-blue-800/30 rounded-lg hover:bg-blue-900/20 transition-colors">
                <input
                  type="checkbox"
                  checked={selectedCapabilities.includes(capability)}
                  on:change={() => handleCapabilityToggle(capability)}
                  class="mt-0.5 rounded bg-gray-700 border-gray-600 text-blue-600 focus:ring-blue-500"
                />
                <div class="flex-1 min-w-0">
                  <div class="text-sm font-medium text-blue-300">
                    {$getCapabilityDisplay(capability)}
                  </div>
                  <div class="text-xs text-gray-400 mt-1">
                    {$getCapabilityDescription(capability)}
                  </div>
                </div>
              </label>
            {/each}
          </div>
        </div>
      {/if}
      
      <!-- Other capabilities -->
      {#if otherCapabilities.length > 0}
        <div>
          <h4 class="text-sm font-medium text-gray-300 mb-3 flex items-center gap-2">
            <span class="w-2 h-2 bg-gray-500 rounded-full"></span>
            Other Available Capabilities
          </h4>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
            {#each otherCapabilities as capability}
              <label class="flex items-start space-x-3 cursor-pointer p-3 bg-gray-700/20 border border-gray-600 rounded-lg hover:bg-gray-700/30 transition-colors">
                <input
                  type="checkbox"
                  checked={selectedCapabilities.includes(capability.id)}
                  on:change={() => handleCapabilityToggle(capability.id)}
                  class="mt-0.5 rounded bg-gray-700 border-gray-600 text-blue-600 focus:ring-blue-500"
                />
                <div class="flex-1 min-w-0">
                  <div class="text-sm font-medium text-gray-300">
                    {$getCapabilityDisplay(capability.id)}
                  </div>
                  <div class="text-xs text-gray-400 mt-1">
                    {$getCapabilityDescription(capability.id)}
                  </div>
                </div>
              </label>
            {/each}
          </div>
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
    {#if selectedCapabilities.length === 0}
      <p class="text-sm text-yellow-400">
        ⚠️ No capabilities selected. Consider selecting at least SSH Access for remote management.
      </p>
    {:else}
      <p class="text-sm text-gray-400">
        <span class="font-medium text-white">{selectedCapabilities.length}</span> 
        capabilit{selectedCapabilities.length === 1 ? 'y' : 'ies'} selected
        {#if suggestedCapabilities.length > 0}
          • <span class="text-blue-400">{suggestedCapabilities.filter(cap => selectedCapabilities.includes(cap)).length} of {suggestedCapabilities.length} suggested</span>
        {/if}
      </p>
    {/if}
  </div>
</div>