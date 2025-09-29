<script lang="ts">
	import { formatId, formatTimestamp } from '$lib/shared/utils/formatting';
  import { Calendar, Clock, Hash, ChevronDown, ChevronRight } from 'lucide-svelte';
  
  export let id: string;
  export let createdAt: string;
  export let updatedAt: string;
  export let entity: any = null;
    
  let isJsonExpanded = false;
  
  // Copy ID to clipboard
  async function copyId() {
    try {
      await navigator.clipboard.writeText(id);
    } catch (error) {
      console.warn('Failed to copy ID to clipboard:', error);
    }
  }
  
  // Copy JSON to clipboard
  async function copyJson() {
    if (!entity) return;
    try {
      await navigator.clipboard.writeText(JSON.stringify(entity, null, 2));
    } catch (error) {
      console.warn('Failed to copy JSON to clipboard:', error);
    }
  }
  
  function toggleJson() {
    isJsonExpanded = !isJsonExpanded;
  }
</script>

<div class="border-t border-gray-700 pt-6">
  <div class="bg-gray-800/50 rounded-lg p-4">
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      
      <!-- ID -->
      <div class="flex items-center space-x-3">
        <div class="flex-shrink-0">
          <Hash class="h-5 w-5 text-gray-400" />
        </div>
        <div class="min-w-0 flex-1">
          <p class="text-sm font-medium text-gray-300">ID</p>
          <button 
            type="button"
            class="text-sm text-gray-400 hover:text-white transition-colors cursor-pointer font-mono truncate block max-w-full"
            title={`${id} (Click to copy)`}
            on:click={copyId}
          >
            {formatId(id)}
          </button>
        </div>
      </div>
      
      <!-- Created -->
      <div class="flex items-center space-x-3">
        <div class="flex-shrink-0">
          <Calendar class="h-5 w-5 text-gray-400" />
        </div>
        <div class="min-w-0 flex-1">
          <p class="text-sm font-medium text-gray-300">Created</p>
          <p class="text-sm text-gray-400" title={createdAt}>
            {formatTimestamp(createdAt)}
          </p>
        </div>
      </div>
      
      <!-- Updated -->
      <div class="flex items-center space-x-3">
        <div class="flex-shrink-0">
          <Clock class="h-5 w-5 text-gray-400" />
        </div>
        <div class="min-w-0 flex-1">
          <p class="text-sm font-medium text-gray-300">Updated</p>
          <p class="text-sm text-gray-400" title={updatedAt}>
            {formatTimestamp(updatedAt)}
          </p>
        </div>
      </div>
      
    </div>
    
    <!-- JSON Entity Section -->
    {#if entity}
      <div class="mt-6 pt-4 border-t border-gray-700">
        <button 
          type="button"
          class="flex items-center space-x-2 text-sm font-medium text-gray-300 hover:text-white transition-colors w-full text-left"
          on:click={toggleJson}
        >
          {#if isJsonExpanded}
            <ChevronDown class="h-4 w-4" />
          {:else}
            <ChevronRight class="h-4 w-4" />
          {/if}
          <span>JSON</span>
        </button>
        
        {#if isJsonExpanded}
          <div class="mt-3 relative">
            <div class="absolute top-2 right-2 z-10">
              <button
                type="button"
                class="text-xs text-gray-400 hover:text-white transition-colors bg-gray-900 px-2 py-1 rounded border border-gray-600"
                title="Copy JSON to clipboard"
                on:click={copyJson}
              >
                Copy
              </button>
            </div>
            <pre class="bg-gray-900 rounded-md p-4 overflow-auto text-sm text-gray-300 font-mono border border-gray-600"><code>{JSON.stringify(entity, null, 2)}</code></pre>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>