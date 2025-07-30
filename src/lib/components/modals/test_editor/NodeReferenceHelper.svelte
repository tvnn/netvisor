<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { NetworkNode } from '$lib/types';

  export let nodeOptions: NetworkNode[] = [];
  export let field: string;

  const dispatch = createEventDispatcher<{
    insertReference: { field: string; reference: string };
  }>();

  interface NodeReferenceOption {
    value: string;
    label: string;
  }

  function getNodeReferenceOptions(): NodeReferenceOption[] {
    return nodeOptions.flatMap((node): NodeReferenceOption[] => [
      { value: `{{${node.id}}}`, label: `${node.name} (auto)` },
      { value: `{{${node.id}.domain}}`, label: `${node.name} (domain)` },
      { value: `{{${node.id}.ip}}`, label: `${node.name} (IP)` },
      { value: `{{${node.id}.defaultPort}}`, label: `${node.name} (port)` }
    ]);
  }

  function handleReferenceSelect(event: Event): void {
    const target = event.target as HTMLSelectElement;
    const reference = target.value;
    
    if (reference) {
      dispatch('insertReference', { field, reference });
      target.value = ''; // Reset select
    }
  }
</script>

<select
  on:change={handleReferenceSelect}
  class="absolute right-0 top-0 h-full bg-gray-600 border-l border-gray-500 text-white text-xs px-2 rounded-r opacity-75 hover:opacity-100"
  title="Insert node reference"
>
  <option value="">Node ref...</option>
  {#each getNodeReferenceOptions() as option}
    <option value={option.value}>{option.label}</option>
  {/each}
</select>