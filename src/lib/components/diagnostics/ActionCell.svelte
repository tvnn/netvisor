<script lang="ts">
  import { Eye, Trash2 } from 'lucide-svelte';
	import type { DiagnosticExecution } from './types';
  
  export let execution: DiagnosticExecution;
  export let deletingId: string | null;
  export let onViewDetails: (execution: DiagnosticExecution) => void;
  export let onDelete: (execution: DiagnosticExecution) => void;
</script>

<div class="flex items-center gap-2 whitespace-nowrap">
  <button
    on:click={() => onViewDetails(execution)}
    class="p-2 text-gray-400 hover:text-white hover:bg-gray-700 rounded transition-colors"
    title="View Details"
  >
    <Eye size={16} />
  </button>
  <!-- <button
    on:click={() => onDelete(execution)}
    disabled={deletingId === execution.id}
    class="p-2 text-gray-400 hover:text-red-400 hover:bg-red-900/20 rounded transition-colors disabled:opacity-50"
    title="Delete"
  >
    <Trash2 size={16} class="{deletingId === execution.id ? 'animate-pulse' : ''}" />
  </button> -->
</div>