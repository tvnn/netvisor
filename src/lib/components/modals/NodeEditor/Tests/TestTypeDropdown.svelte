<script lang="ts">
  import { ChevronDown, Server, CheckCircle, AlertTriangle, XCircle } from 'lucide-svelte';
  import { testTypes } from '$lib/api/registry';
  
  export let selectedTestType: string;
  export let onTestTypeChange: (testType: string) => void;
  export let schemaCache: Map<string, any> = new Map(); // Schema cache from parent
  
  let isOpen = false;
  let dropdownElement: HTMLDivElement;
  
  $: selectedTestInfo = $testTypes.find(t => t.id === selectedTestType);
  
  function handleSelect(testType: string) {
    onTestTypeChange(testType);
    isOpen = false;
  }
  
  function handleClickOutside(event: MouseEvent) {
    if (dropdownElement && !dropdownElement.contains(event.target as Node)) {
      isOpen = false;
    }
  }
  
  function getCompatibilityInfo(testTypeId: string) {
    const schema = schemaCache.get(testTypeId);
    if (!schema) return null;
    
    return {
      status: schema.compatibility,
      reason: schema.compatibility_reason
    };
  }
  
  function getCompatibilityIcon(status: string) {
    switch (status) {
      case 'Compatible':
        return CheckCircle;
      case 'Conditional':
        return AlertTriangle;
      case 'Incompatible':
        return XCircle;
      default:
        return null;
    }
  }
  
  function getCompatibilityColor(status: string) {
    switch (status) {
      case 'Compatible':
        return 'text-green-400';
      case 'Conditional':
        return 'text-yellow-400';
      case 'Incompatible':
        return 'text-red-400';
      default:
        return 'text-gray-400';
    }
  }
</script>

<svelte:window on:click={handleClickOutside} />

<div class="relative" bind:this={dropdownElement}>
  <div class="block text-sm font-medium text-gray-300 mb-2">
    Test Type
  </div>
  
  <!-- Dropdown Trigger -->
  <button
    type="button"
    on:click={() => isOpen = !isOpen}
    class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white 
           focus:outline-none focus:ring-2 focus:ring-blue-500 flex items-center justify-between"
  >
    <div class="flex items-center gap-3">
      {#if selectedTestInfo}
        <div class="w-6 h-6 rounded bg-gray-600 flex items-center justify-center">
          <Server class="w-3 h-3 {selectedTestInfo.color}" />
        </div>
        <span>{selectedTestInfo.display_name}</span>
      {:else}
        <span class="text-gray-400">Select a test type...</span>
      {/if}
    </div>
    <ChevronDown class="w-4 h-4 text-gray-400 transition-transform {isOpen ? 'rotate-180' : ''}" />
  </button>
  
  <!-- Test Description - inline text below dropdown -->
  {#if selectedTestInfo}
    <div class="mt-2">
      <p class="text-sm text-gray-400">
        {selectedTestInfo.description}
        <span class="ml-2 px-2 py-0.5 text-xs bg-gray-600 text-gray-300 rounded">
          {selectedTestInfo.category}
        </span>
      </p>
    </div>
  {/if}
  
  <!-- Dropdown Menu - positioned to overlay the description -->
  {#if isOpen}
    <div class="absolute z-50 w-full bg-gray-700 border border-gray-600 rounded-md shadow-lg max-h-96 overflow-y-auto" style="top: calc(100% + 0.5rem);">
      {#each $testTypes as testType}
        {@const compatibilityInfo = getCompatibilityInfo(testType.id)}
        
        <button
          type="button"
          on:click={() => handleSelect(testType.id)}
          class="w-full px-3 py-3 text-left hover:bg-gray-600 transition-colors border-b border-gray-600 last:border-b-0
                 {compatibilityInfo?.status === 'Incompatible' ? 'opacity-60' : ''}"
        >
          <div class="flex items-start gap-3">
            <div class="w-8 h-8 rounded-lg bg-gray-600 flex items-center justify-center mt-0.5">
              <Server class="w-4 h-4 {testType.color}" />
            </div>
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2 flex-wrap">
                <h4 class="font-medium text-white">{testType.display_name}</h4>
                
                <span class="inline-block px-2 py-1 text-xs bg-gray-600 text-gray-300 rounded">
                  {testType.category}
                </span>
                
                {#if compatibilityInfo}
                  <span class="inline-block px-2 py-1 text-xs rounded
                              {compatibilityInfo.status === 'Compatible' ? 'bg-green-900/30 text-green-300' :
                               compatibilityInfo.status === 'Conditional' ? 'bg-yellow-900/30 text-yellow-300' :
                               'bg-red-900/30 text-red-300'}">
                    {compatibilityInfo.status}
                  </span>
                {/if}
              </div>
              
              <p class="text-sm text-gray-400 mt-1 line-clamp-2">{testType.description}</p>
              
              {#if compatibilityInfo?.reason}
                <p class="text-xs mt-1 {getCompatibilityColor(compatibilityInfo.status)}">
                  {compatibilityInfo.reason}
                </p>
              {/if}
            </div>
          </div>
        </button>
      {/each}
    </div>
  {/if}
</div>