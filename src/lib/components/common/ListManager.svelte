<script lang="ts">
  import { ArrowUp, ArrowDown, Trash2, Plus, Edit } from 'lucide-svelte';
  
  export let label: string;
  export let helpText: string = '';
  export let items: any[] = [];
  export let availableOptions: ListOption[] = [];
  export let placeholder: string = 'Select an item to add';
  export let required: boolean = false;
  export let allowReorder: boolean = true;
  export let allowEdit: boolean = false;
  export let allowDirectAdd: boolean = true;
  export let getDisplayName: (item: any) => string = (item) => item?.toString() || '';
  export let getDisplayDetails: (item: any) => string = () => '';
  export let getDisplayBadges: (item: any) => Badge[] = () => [];
  export let onEdit: (item: any, index: number) => void = () => {};
  export let highlightedIndex: number = -1;
  export let onAdd: () => void = () => {};
  export let emptyMessage: string = '';
  export let error: string = '';
  
  interface ListOption {
    id: string;
    label: string;
    subtitle?: string;
  }
  
  interface Badge {
    text: string;
    color: string;
    bgColor?: string;
  }
  
  let selectedItemId = '';
  
  $: filteredOptions = availableOptions.filter(option => !items.some(item => 
    typeof item === 'string' ? item === option.id : item.id === option.id
  ));
  
  $: computedEmptyMessage = emptyMessage || `No ${label.toLowerCase()} added yet`;
  
  function addItem() {
    if (selectedItemId && !items.some(item => 
      typeof item === 'string' ? item === selectedItemId : item.id === selectedItemId
    )) {
      items = [...items, selectedItemId];
      selectedItemId = '';
    }
  }
  
  function removeItem(index: number) {
    items = items.filter((_, i) => i !== index);
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
  <div class="flex justify-between items-start mb-2 gap-4">
    <div class="flex-1 min-w-0">
      <div class="block text-sm font-medium text-gray-300">
        {label}
        {#if required}<span class="text-red-400">*</span>{/if}
      </div>
      {#if helpText}
        <p class="text-sm text-gray-400 mt-1">
          {helpText}
        </p>
      {/if}
    </div>
    
    {#if !allowDirectAdd}
      <button
        type="button"
        on:click={onAdd}
        class="flex items-center gap-2 px-3 py-2 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors flex-shrink-0"
      >
        <Plus size={16} />
        {placeholder}
      </button>
    {/if}
  </div>
  
  <!-- Add Item Section -->
  {#if allowDirectAdd && availableOptions.length > 0}
    <div class="flex gap-2 mb-3 mt-4">
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
  {/if}
  
  <!-- Current Items -->
  {#if items.length > 0}
    <div class="space-y-2 mb-3">
      {#each items as item, index}
        {@const displayName = getDisplayName(item)}
        {@const displayDetails = getDisplayDetails(item)}
        {@const badges = getDisplayBadges(item)}
        {@const isHighlighted = highlightedIndex === index}
        
        <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
        <div 
          class="flex items-center gap-3 p-3 rounded-lg border transition-all {allowEdit ? 'cursor-pointer hover:border-gray-500' : ''} {isHighlighted ? 'bg-blue-900/30 border-blue-500' : 'bg-gray-700/50 border-gray-600'}"
          on:click={allowEdit ? () => onEdit(item, index) : undefined}
          role={allowEdit ? "button" : undefined}
          tabindex={allowEdit ? 0 : undefined}
          on:keydown={allowEdit ? (e) => {
            if (e.key === 'Enter' || e.key === ' ') {
              e.preventDefault();
              onEdit(item, index);
            }
          } : undefined}
          >
          {#if allowReorder}
            <span class="text-gray-400 font-mono text-sm min-w-[2rem]">{index + 1}.</span>
          {/if}
          
          <!-- Item Info -->
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2 mb-1">
              <span class="font-medium text-white">
                {displayName}
              </span>
              {#each badges as badge}
                <span class="text-xs px-2 py-1 rounded {badge.color} {badge.bgColor || 'bg-gray-800'}">
                  {badge.text}
                </span>
              {/each}
            </div>
            {#if displayDetails}
              <div class="text-sm text-gray-400">
                {displayDetails}
              </div>
            {/if}
          </div>
          
          <!-- Actions -->
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
              on:click={() => removeItem(index)}
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
      {computedEmptyMessage}
    </div>
  {/if}
  
  <!-- Error message -->
  {#if error}
    <p class="text-red-400 text-xs mt-1">{error}</p>
  {/if}
</div>