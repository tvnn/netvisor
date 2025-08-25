<script lang="ts">
  import { Eye, Trash2 } from 'lucide-svelte';
  import type { DiagnosticExecution } from './types';
  import { diagnosticsActionsStore } from './store';
  
  export let row: DiagnosticExecution;
</script>

<div class="items-center gap-2 whitespace-nowrap">
  <button
    on:click={() => $diagnosticsActionsStore.handleViewDetails?.(row)}
    class="p-2 text-gray-400 hover:text-white hover:bg-gray-700 rounded transition-colors"
    title="View Details"
  >
    <Eye size={16} />
  </button>
  
  <button
    on:click={() => $diagnosticsActionsStore.handleDelete?.(row)}
    disabled={$diagnosticsActionsStore.deletingId === row.id}
    class="p-2 text-gray-400 hover:text-red-400 hover:bg-red-900/20 rounded transition-colors disabled:opacity-50"
    title="Delete"
  >
    <Trash2 size={16} class="{$diagnosticsActionsStore.deletingId === row.id ? 'animate-pulse' : ''}" />
  </button>
</div>