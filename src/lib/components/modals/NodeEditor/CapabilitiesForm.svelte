<script lang="ts">
  import { onMount } from 'svelte';
  import type { CapabilityRecommendations } from "$lib/types/nodes";
  import type { NodeType, NodeCapability } from "$lib/types/nodes";
  import {
    getAllCapabilities,
    getCapabilityDisplay,
    getCapabilityDescription,
  } from "$lib/config/nodes/capabilities";
  import { nodeActions } from '$lib/stores/nodes';
  
  export let capabilities: NodeCapability[];
  export let nodeType: NodeType;
  export let nodeId: string | undefined = undefined;
  export let preloadedRecommendations: CapabilityRecommendations | null = null;
  
  let recommendations: CapabilityRecommendations | null = null;
  let loading = false;
  
  // Fetch recommendations from backend
  async function fetchRecommendations() {
    if (!nodeId) {
      // For new nodes, no recommendations - just show all capabilities
      recommendations = {
        all_capabilities: getAllCapabilities(),
        current_capabilities: capabilities,
        suggested_capabilities: []
      };
      return;
    }
    
    loading = true;
    try {
      const response = await nodeActions.getCapabilityRecommendations(nodeType);
      if (response) {
        recommendations = response;
        
        // Auto-apply suggested capabilities if none are currently selected
        if (capabilities.length === 0 && response.suggested_capabilities.length > 0) {
          capabilities = [...response.suggested_capabilities];
        }
      } else {
        // Fallback if API fails - no suggestions
        recommendations = {
          all_capabilities: getAllCapabilities(),
          current_capabilities: capabilities,
          suggested_capabilities: []
        };
      }
    } catch (error) {
      console.error('Failed to fetch capability recommendations:', error);
      // Fallback if API fails - no suggestions
      recommendations = {
        all_capabilities: getAllCapabilities(),
        current_capabilities: capabilities,
        suggested_capabilities: []
      };
    } finally {
      loading = false;
    }
  }
  
  function handleCapabilityToggle(capability: NodeCapability) {
    if (capabilities.includes(capability)) {
      capabilities = capabilities.filter(c => c !== capability);
    } else {
      capabilities = [...capabilities, capability];
    }
  }
  
  function applySuggested() {
    if (recommendations && recommendations.suggested_capabilities.length > 0) {
      capabilities = [...recommendations.suggested_capabilities];
    }
  }
  
  // Auto-apply suggestions when they change (for node type changes)
  let lastSuggestions: NodeCapability[] = [];
  $: if (recommendations && JSON.stringify(recommendations.suggested_capabilities) !== JSON.stringify(lastSuggestions)) {
    // Only auto-apply if user hasn't made manual selections or if capabilities are empty
    if (capabilities.length === 0 || capabilities.every(cap => lastSuggestions.includes(cap))) {
      capabilities = [...recommendations.suggested_capabilities];
    }
    lastSuggestions = [...recommendations.suggested_capabilities];
  }
  
  onMount(() => {
    // Use preloaded recommendations if available, otherwise fetch
    if (preloadedRecommendations) {
      recommendations = preloadedRecommendations;
    } else {
      fetchRecommendations();
    }
  });
  
  // Use preloaded recommendations when they become available
  $: if (preloadedRecommendations) {
    recommendations = preloadedRecommendations;
  }
  
  // Only fetch if we don't have preloaded data and node type changes
  $: if (!preloadedRecommendations && nodeType && nodeType !== 'UnknownDevice') {
    fetchRecommendations();
  }
  
  // Computed values
  $: suggestedCapabilities = recommendations?.suggested_capabilities || [];
  $: otherCapabilities = recommendations 
    ? recommendations.all_capabilities.filter(cap => !suggestedCapabilities.includes(cap))
    : [];
</script>

<div class="space-y-4">
  <!-- Header with description -->
  <div>
    <div class="flex items-center justify-between mb-2">
      <div class="flex items-center gap-2">
        <h3 class="text-lg font-medium text-white">Capabilities</h3>
        {#if loading}
          <div class="w-4 h-4 border-2 border-blue-400 border-t-transparent rounded-full animate-spin"></div>
        {/if}
      </div>
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
    <p class="text-sm text-gray-400 mb-4">
      Capabilities help determine which tests are compatible with this node and enable 
      automatic test recommendations. Select the services and access methods available on this device.
    </p>
  </div>
  
  <!-- Capabilities List -->
  {#if recommendations}
    <div class="space-y-4">
      <!-- Suggested capabilities first (if any) -->
      {#if suggestedCapabilities.length > 0}
        <div>
          <h4 class="text-sm font-medium text-blue-300 mb-3 flex items-center gap-2">
            <span class="w-2 h-2 bg-blue-400 rounded-full"></span>
            Suggested for {nodeType}
          </h4>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
            {#each suggestedCapabilities as capability}
              <label class="flex items-start space-x-3 cursor-pointer p-3 bg-blue-900/10 border border-blue-800/30 rounded-lg hover:bg-blue-900/20 transition-colors">
                <input
                  type="checkbox"
                  checked={capabilities.includes(capability)}
                  on:change={() => handleCapabilityToggle(capability)}
                  class="mt-0.5 rounded bg-gray-700 border-gray-600 text-blue-600 focus:ring-blue-500"
                />
                <div class="flex-1 min-w-0">
                  <div class="text-sm font-medium text-blue-300">
                    {getCapabilityDisplay(capability)}
                  </div>
                  <div class="text-xs text-gray-400 mt-1">
                    {getCapabilityDescription(capability)}
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
                  checked={capabilities.includes(capability)}
                  on:change={() => handleCapabilityToggle(capability)}
                  class="mt-0.5 rounded bg-gray-700 border-gray-600 text-blue-600 focus:ring-blue-500"
                />
                <div class="flex-1 min-w-0">
                  <div class="text-sm font-medium text-gray-300">
                    {getCapabilityDisplay(capability)}
                  </div>
                  <div class="text-xs text-gray-400 mt-1">
                    {getCapabilityDescription(capability)}
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
    {#if capabilities.length === 0}
      <p class="text-sm text-yellow-400">
        ⚠️ No capabilities selected. Consider selecting at least SSH Access for remote management.
      </p>
    {:else}
      <p class="text-sm text-gray-400">
        <span class="font-medium text-white">{capabilities.length}</span> 
        capabilit{capabilities.length === 1 ? 'y' : 'ies'} selected
        {#if suggestedCapabilities.length > 0}
          • <span class="text-blue-400">{suggestedCapabilities.filter(cap => capabilities.includes(cap)).length} of {suggestedCapabilities.length} suggested</span>
        {/if}
      </p>
    {/if}
  </div>
</div>