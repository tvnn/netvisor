<script lang="ts">
	import { formatId, formatTimestamp } from '$lib/shared/utils/formatting';
  import { Calendar, Clock, Hash } from 'lucide-svelte';
  
  export let id: string;
  export let createdAt: string;
  export let updatedAt: string;
  export let title: string = "Metadata";
    
  // Copy ID to clipboard
  async function copyId() {
    try {
      await navigator.clipboard.writeText(id);
    } catch (error) {
      console.warn('Failed to copy ID to clipboard:', error);
    }
  }
</script>

<div class="border-t border-gray-700 pt-6">
  <h3 class="text-lg font-medium text-white mb-4">{title}</h3>
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
  </div>
</div>