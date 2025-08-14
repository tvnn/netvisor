<script lang="ts">
  import { ArrowUp, ArrowDown, Trash2, Plus } from 'lucide-svelte';
  
  export let label: string;
  export let items: string[] = [];
  export let availableOptions: ListOption[] = [];
  export let placeholder: string = 'Select an item to add';
  export let required: boolean = false;
  export let allowReorder: boolean = true;
  export let getDisplayName: (id: string) => string = (id) => id;
  export let error: string = '';
  
  interface ListOption {
    id: string;
    label: string;
    subtitle?: string;
  }
  
  let selectedItemId = '';
  
  $: filteredOptions = availableOptions.filter(option => !items.includes(option.id));
  
  function addItem() {
    if (selectedItemId && !items.includes(selectedItemId)) {
      items = [...items, selectedItemId];
      selectedItemId = '';
    }
  }
  
  function removeItem(itemId: string) {
    items = items.filter(id => id !== itemId);
  }
  
  function moveItemUp(index: number) {
    if (index > 0 && allowReorder) {
      const newItems = [...items];
      [newItems[index - 1], newItems[index]] = [newItems[index], newItems[index - 1]];
      items = newItems;
    }
  }
  
  function moveItemDown(index: number) {
    if (index < items.length - 1 && allowReorder) {
      const newItems = [...items];
      [newItems[index], newItems[index + 1]] = [newItems[index + 1], newItems[index]];
      items = newItems;
    }
  }
</script>

<div>
  <label class="block text-sm font-medium text-gray-300 mb-2">
    {label}
    {#if required}<span class="text-red-400">*</span>{/if}
  </label>
  
  <!-- Add Item -->
  <div class="flex gap-2 mb-3">
    <select
      bind:value={selectedItemId}
      class="flex-1 px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
    >
      <option value="">{placeholder}</option>
      {#each filteredOptions as option}
        <option value={option.id}>
          {option.label}
          {#if option.subtitle}({option.subtitle}){/if}
        </option>
      {/each}
    </select>
    <button
      type="button"
      on:click={addItem}
      disabled={!selectedItemId}
      class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
    >
      <Plus size={16} />
      Add
    </button>
  </div>
  
  <!-- Current Items -->
  {#if items.length > 0}
    <div class="space-y-2 mb-3">
      {#each items as itemId, index}
        <div class="flex items-center gap-2 bg-gray-700/50 rounded-lg p-3">
          {#if allowReorder}
            <span class="text-gray-400 font-mono text-sm min-w-[2rem]">{index + 1}.</span>
          {/if}
          <span class="flex-1 text-white">{getDisplayName(itemId)}</span>
          
          <div class="flex items-center gap-1">
            {#if allowReorder}
              <button
                type="button"
                on:click={() => moveItemUp(index)}
                disabled={index === 0}
                class="p-1 text-gray-400 hover:text-white hover:bg-gray-600 rounded disabled:opacity-30 disabled:cursor-not-allowed"
                title="Move up"
              >
                <ArrowUp size={16} />
              </button>
              
              <button
                type="button"
                on:click={() => moveItemDown(index)}
                disabled={index === items.length - 1}
                class="p-1 text-gray-400 hover:text-white hover:bg-gray-600 rounded disabled:opacity-30 disabled:cursor-not-allowed"
                title="Move down"
              >
                <ArrowDown size={16} />
              </button>
            {/if}
            
            <button
              type="button"
              on:click={() => removeItem(itemId)}
              class="p-1 text-gray-400 hover:text-red-400 hover:bg-red-900/20 rounded"
              title="Remove"
            >
              <Trash2 size={16} />
            </button>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="text-gray-500 text-sm mb-3 p-3 bg-gray-700/30 rounded-lg text-center">
      No {label.toLowerCase()} added yet
    </div>
  {/if}
  
  <!-- Error message -->
  {#if error}
    <p class="text-red-400 text-xs mt-1">{error}</p>
  {/if}
</div>