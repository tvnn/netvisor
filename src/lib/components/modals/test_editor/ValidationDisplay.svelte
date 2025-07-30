<script lang="ts">
  import { AlertCircle } from 'lucide-svelte';

  export let errors: string[] = [];
</script>

{#if errors.length > 0}
  <div class="bg-red-900/30 border border-red-700/50 rounded-lg p-4">
    <div class="flex items-center gap-2 mb-2">
      <AlertCircle class="w-4 h-4 text-red-400" />
      <span class="text-red-400 font-medium">Validation Errors</span>
    </div>
    <ul class="text-sm text-red-300 space-y-1">
      {#each errors as error}
        <li>• {error}</li>
      {/each}
    </ul>
  </div>
{/if}