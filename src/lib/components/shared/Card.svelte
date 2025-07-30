<!-- src/lib/components/shared/Card.svelte -->
<script lang="ts">
  export let title: string;
  export let description: string = '';
  export let metadata: Array<{label: string, value: string}> = [];
  export let onEdit: (() => void) | null = null;
  export let onCopy: (() => void) | null = null;
  export let onDelete: (() => void) | null = null;
  export let onRun: (() => void) | null = null; // Optional run action for tests
  
  import { Edit, Copy, Trash2, Play } from 'lucide-svelte';
</script>

<div class="card-modern hover:bg-gray-700/50 transition-all duration-200 flex flex-col h-full">
  <!-- Content area that grows to push buttons to bottom -->
  <div class="flex-1">
    <!-- Header with title and description -->
    <div class="mb-4">
      <h4 class="font-medium text-white mb-1">{title}</h4>
      {#if description}
        <p class="text-sm text-gray-400 line-clamp-2">{description}</p>
      {/if}
    </div>
    
    <!-- Metadata -->
    {#if metadata.length > 0}
      <div class="space-y-2 text-sm">
        {#each metadata as item}
          <div class="flex justify-between">
            <span class="text-gray-400">{item.label}:</span>
            <span class="text-white font-mono">{item.value}</span>
          </div>
        {/each}
      </div>
    {/if}
  </div>
  
  <!-- Action buttons at bottom - full width with equal spacing -->
  <div class="flex justify-between pt-4 border-t border-gray-700 mt-4">
    {#if onRun}
      <button
        class="flex-1 mx-1 p-2 hover:bg-green-700 rounded text-gray-400 hover:text-green-400 transition-colors flex items-center justify-center"
        on:click={onRun}
        title="Run test"
      >
        <Play class="w-4 h-4" />
      </button>
    {/if}
    {#if onEdit}
      <button
        class="flex-1 mx-1 p-2 hover:bg-gray-700 rounded text-gray-400 hover:text-white transition-colors flex items-center justify-center"
        on:click={onEdit}
        title="Edit"
      >
        <Edit class="w-4 h-4" />
      </button>
    {/if}
    {#if onCopy}
      <button
        class="flex-1 mx-1 p-2 hover:bg-gray-700 rounded text-gray-400 hover:text-white transition-colors flex items-center justify-center"
        on:click={onCopy}
        title="Duplicate"
      >
        <Copy class="w-4 h-4" />
      </button>
    {/if}
    {#if onDelete}
      <button
        class="flex-1 mx-1 p-2 hover:bg-red-700 rounded text-gray-400 hover:text-red-400 transition-colors flex items-center justify-center"
        on:click={onDelete}
        title="Delete"
      >
        <Trash2 class="w-4 h-4" />
      </button>
    {/if}
  </div>
</div>

<style>
  .line-clamp-2 {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>