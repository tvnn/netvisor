<script lang="ts">
  import { Plus, Trash2, Move, ChevronDown, ChevronRight } from 'lucide-svelte';
  import { createEventDispatcher } from 'svelte';
  import type { Layer, Check, NetworkNode } from '$lib/types';

  import CheckEditor from './CheckEditor.svelte';

  export let layer: Layer;
  export let nodeOptions: NetworkNode[] = [];
  export let expanded: boolean = false;
  export let canMoveUp: boolean = false;
  export let canMoveDown: boolean = false;

  const dispatch = createEventDispatcher<{
    layerUpdate: Layer;
    toggleExpanded: void;
    moveUp: void;
    moveDown: void;
    remove: void;
  }>();

  let expandedChecks: Set<string> = new Set();

  function addCheck(): void {
    const newCheck: Check = {
      type: 'connectivityCheck',
      config: {}
    };

    layer.checks = [...layer.checks, newCheck];
    dispatchUpdate();
  }

  function removeCheck(checkIndex: number): void {
    layer.checks = layer.checks.filter((_, i) => i !== checkIndex);
    dispatchUpdate();
  }

  function handleCheckUpdate(event: CustomEvent<{ index: number; check: Check }>): void {
    const { index: checkIndex, check } = event.detail;
    layer.checks[checkIndex] = check;
    layer.checks = layer.checks; // Trigger reactivity
    dispatchUpdate();
  }

  function toggleCheck(checkId: string): void {
    if (expandedChecks.has(checkId)) {
      expandedChecks.delete(checkId);
    } else {
      expandedChecks.add(checkId);
    }
    expandedChecks = expandedChecks;
  }

  function dispatchUpdate(): void {
    dispatch('layerUpdate', layer);
  }

  // Update layer name and description
  function updateLayerName(event: Event): void {
    const target = event.target as HTMLInputElement;
    layer.name = target.value;
    dispatchUpdate();
  }

  function updateLayerDescription(event: Event): void {
    const target = event.target as HTMLTextAreaElement;
    layer.description = target.value;
    dispatchUpdate();
  }
</script>

<div class="border border-gray-600 rounded-lg bg-gray-800/50">
  <!-- Layer Header -->
  <div class="flex items-center justify-between p-3 border-b border-gray-600">
    <div class="flex items-center gap-3">
      <button
        type="button"
        on:click={() => dispatch('toggleExpanded')}
        class="text-gray-400 hover:text-white transition-colors"
      >
        {#if expanded}
          <ChevronDown class="w-4 h-4" />
        {:else}
          <ChevronRight class="w-4 h-4" />
        {/if}
      </button>

      <input
        type="text"
        value={layer.name}
        on:input={updateLayerName}
        class="bg-transparent text-white font-medium border-none outline-none focus:bg-gray-700 rounded px-2 py-1"
        placeholder="Layer name"
      />

      <span class="text-xs text-gray-400">
        {layer.checks.length} check{layer.checks.length !== 1 ? 's' : ''}
      </span>
    </div>

    <div class="flex items-center gap-2">
      <button
        type="button"
        on:click={() => dispatch('moveUp')}
        disabled={!canMoveUp}
        class="p-1 hover:bg-gray-700 rounded text-gray-400 hover:text-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        <Move class="w-3 h-3 rotate-180" />
      </button>
      <button
        type="button"
        on:click={() => dispatch('moveDown')}
        disabled={!canMoveDown}
        class="p-1 hover:bg-gray-700 rounded text-gray-400 hover:text-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        <Move class="w-3 h-3" />
      </button>
      <button
        type="button"
        on:click={() => dispatch('remove')}
        class="p-1 hover:bg-red-700 rounded text-gray-400 hover:text-red-400 transition-colors"
      >
        <Trash2 class="w-3 h-3" />
      </button>
    </div>
  </div>

  <!-- Layer Content -->
  {#if expanded}
    <div class="p-3 space-y-3">
      <!-- Layer Description -->
      {#if layer.description !== undefined}
        <div>
          <label for="layer-description-{layer.id}" class="block text-xs font-medium text-gray-300 mb-1">
            Description
          </label>
          <textarea
            id="layer-description-{layer.id}"
            value={layer.description}
            on:input={updateLayerDescription}
            rows="2"
            class="w-full bg-gray-700 border border-gray-600 text-white rounded px-2 py-1 text-sm focus:ring-2 focus:ring-blue-500"
            placeholder="Layer description..."
          ></textarea>
        </div>
      {/if}

      <!-- Add Check Button -->
      <button
        type="button"
        on:click={addCheck}
        class="w-full flex items-center justify-center gap-2 py-2 border-2 border-dashed border-gray-600 hover:border-gray-500 text-gray-400 hover:text-gray-300 rounded-lg transition-colors"
      >
        <Plus class="w-4 h-4" />
        Add Check
      </button>

      <!-- Checks -->
      <div class="space-y-3">
        {#each layer.checks as check, checkIndex}
          <CheckEditor
            {check}
            index={checkIndex}
            {nodeOptions}
            expanded={expandedChecks.has(`${layer.id}-${checkIndex}`)}
            on:checkUpdate={handleCheckUpdate}
            on:toggleExpanded={() => toggleCheck(`${layer.id}-${checkIndex}`)}
            on:remove={() => removeCheck(checkIndex)}
          />
        {/each}
      </div>
    </div>
  {/if}
</div>