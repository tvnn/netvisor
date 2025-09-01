<script lang="ts">
  import { ArrowUp, ArrowDown, Trash2, Plus, Edit } from 'lucide-svelte';
  import RichSelect from './RichSelect.svelte';
	import Tag from '../data/Tag.svelte';
	import ListSelectItem from './ListSelectItem.svelte';
	import type { TagProps } from '../data/types';
  

  // Global
  export let label: string;
  export let helpText: string = '';
  export let placeholder: string = 'Select an item to add';
  export let required: boolean = false;
  export let allowReorder: boolean = true;
  export let allowDirectAdd: boolean = true;
  export let highlightedIndex: number = -1;
  export let emptyMessage: string = '';
  export let error: string = '';

  // Options (dropdown)
  export let options: any[] = [];
  export let getOptionId: (item: any) => string;

  // Options display
  export let getOptionIcon: (item: any) => any | null = (item) => null;
  export let getOptionIconColor: (item: any) => string | null = (item) => null;
  export let getOptionTags: (item: any) => TagProps[] = (item) => [];
  export let getOptionLabel: (item: any) => string | null = (item) => null
  export let getOptionDescription: (item: any) => string | null = (item) => null
  export let getOptionIsDisabled: (item: any) => boolean = (item) => false

  // Items
  export let items: any[] = [];
  export let getItemId: (item: any) => string;

  // Item interaction
  export let allowItemEdit: ((item: any) => boolean) = (item) => true;
  export let allowItemRemove: ((item: any) => boolean) = (item) => true;

  // Item display
  export let getItemIcon: (item: any) => any | null = (item) => null;
  export let getItemIconColor: (item: any) => string | null = (item) => null;
  export let getItemTags: (item: any) => TagProps[] = (item) => [];
  export let getItemLabel: (item: any) => string | null = (item) => null
  export let getItemDescription: (item: any) => string | null = (item) => null
  
  // Interaction handlers
  export let onEdit: (item: any, index: number) => void = () => {};
  export let onAdd: (selectOptionId: string) => void = () => {};
  export let onMoveUp: (fromIndex: number, toIndex: number) => void = () => {};
  export let onMoveDown: (fromIndex: number, toIndex: number) => void = () => {};
  export let onRemove: (index: number) => void = () => {};

  
  let selectedOptionId = '';
  
  $: computedEmptyMessage = emptyMessage || `No ${label.toLowerCase()} added yet`;
  
  function addItem() {
    if (selectedOptionId && !items.some(item => {
      const itemId = getItemId(item)
      return itemId === selectedOptionId;
    })) {
      items = [...items, selectedOptionId];
      selectedOptionId = '';
    }
  }
  
  function removeItem(index: number) {
    items = items.filter((_, i) => i !== index);
    onRemove(index);
  }
  
  function moveItemUp(index: number) {
    if (index > 0 && allowReorder) {
      const newItems = [...items];
      [newItems[index - 1], newItems[index]] = [newItems[index], newItems[index - 1]];
      items = newItems;
      onMoveUp(index, index - 1);
    }
  }
  
  function moveItemDown(index: number) {
    if (index < items.length - 1 && allowReorder) {
      const newItems = [...items];
      [newItems[index], newItems[index + 1]] = [newItems[index + 1], newItems[index]];
      items = newItems;
      onMoveDown(index, index + 1); 
    }
  }

  function handleSelectChange(value: string) {
    selectedOptionId = value;
    addItem();
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
        on:click={() => onAdd(selectedOptionId)}
        class="flex items-center gap-2 px-3 py-2 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors flex-shrink-0"
      >
        <Plus size={16} />
        {placeholder}
      </button>
    {/if}
  </div>
  
  <!-- Add Item Section with RichSelect -->
  {#if allowDirectAdd && items.length > 0}
    <div class="mb-3 mt-4">
      <div class="flex gap-2">
        <!-- RichSelect Component -->
        <div class="flex-1">
          <RichSelect
            selectedValue={selectedOptionId}
            {options}
            {placeholder}
            onSelect={handleSelectChange}
            {getOptionId}
            {getOptionIcon}
            {getOptionIconColor}
            {getOptionLabel}
            {getOptionDescription}
            {getOptionTags}
            {getOptionIsDisabled}
          />
        </div>
      </div>
    </div>
  {/if}
  
  <!-- Current Items -->
  {#if items.length > 0}
    <div class="space-y-2 mb-3">
      {#each items as item, index}
        {@const isHighlighted = highlightedIndex === index}
        
        <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
        <div 
          class="flex items-center gap-3 p-3 rounded-lg border transition-all {allowItemEdit(item) ?
            'cursor-pointer hover:bg-gray-700/30 hover:border-gray-500' : ''} {
            isHighlighted ? 'bg-blue-900/20 border-blue-500' : 'bg-gray-700/20 border-gray-600'
          }"
          on:click={() => allowItemEdit(item) && onEdit(item, index)}
          tabindex={allowItemEdit(item) ? 0 : -1}
          role={allowItemEdit(item) ? 'button' : undefined}
        >
          
          <ListSelectItem
            item={item}
            getIcon={getItemIcon}
            getIconColor={getItemIconColor}
            getTags={getItemTags}
            getLabel={getItemLabel}
            getDescription={getItemDescription} />
          
          <!-- Action Buttons -->
          <div class="flex items-center gap-1">
            {#if allowItemEdit(item)}
              <button
                type="button"
                on:click|stopPropagation={() => onEdit(item, index)}
                class="p-1 text-gray-400 hover:text-white transition-colors"
                title="Edit"
              >
                <Edit size={16} />
              </button>
            {/if}
            
            {#if allowReorder}
              <button
                type="button"
                on:click|stopPropagation={() => moveItemUp(index)}
                disabled={index === 0}
                class="p-1 text-gray-400 hover:text-white transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
                title="Move up"
              >
                <ArrowUp size={16} />
              </button>
              
              <button
                type="button"
                on:click|stopPropagation={() => moveItemDown(index)}
                disabled={index === items.length - 1}
                class="p-1 text-gray-400 hover:text-white transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
                title="Move down"
              >
                <ArrowDown size={16} />
              </button>
            {/if}
            
            {#if allowItemRemove(item)}
              <button
                type="button"
                on:click|stopPropagation={() => removeItem(index)}
                class="p-1 text-gray-400 hover:text-red-400 transition-colors"
                title="Remove"
              >
                <Trash2 size={16} />
              </button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {:else if computedEmptyMessage}
    <div class="text-center py-4 text-gray-400 text-sm bg-gray-700/10 border border-gray-600 rounded-lg border-dashed">
      {computedEmptyMessage}
    </div>
  {/if}
  
  <!-- Error Message -->
  {#if error}
    <div class="text-red-400 text-sm mt-2">
      {error}
    </div>
  {/if}
</div>