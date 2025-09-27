<script lang="ts" generics="T, V">
  import { ArrowUp, ArrowDown, Trash2, Plus, Edit } from 'lucide-svelte';
  import RichSelect from './RichSelect.svelte';
  import ListSelectItem from './ListSelectItem.svelte';
	import type { EntityDisplayComponent } from './types';
	import type { FormApi } from '../types';

  // Global
  export let label: string;
  export let helpText: string = '';
  export let placeholder: string = 'Select an item to add';
  export let required: boolean = false;
  export let allowReorder: boolean = true;
  export let allowCreateNew: boolean = false;
  export let createNewLabel: string = 'Add New';
  export let highlightedIndex: number = -1;
  export let emptyMessage: string = '';
  export let error: string = '';

  // Options (dropdown)
  export let options: V[] = [];
  export let optionDisplayComponent: EntityDisplayComponent<V>;
  export let showSearch: boolean = false;

  // Items
  export let items: T[] = [];
  export let itemDisplayComponent: EntityDisplayComponent<T>;

  // Item interaction
  export let allowDuplicates: boolean = false;
  export let allowItemEdit: ((item: T) => boolean) = (item) => true;
  export let allowItemRemove: ((item: T) => boolean) = (item) => true;
  
  // Interaction handlers
  export let onCreateNew: (() => void) | null = null;
  export let onEdit: (item: T, index: number) => void = () => {};
  export let onAdd: (selectOptionId: string) => void = () => {};
  export let onMoveUp: (fromIndex: number, toIndex: number) => void = () => {};
  export let onMoveDown: (fromIndex: number, toIndex: number) => void = () => {};
  export let onRemove: (index: number) => void = () => {};

  // Internal state
  let selectedOptionId = '';
  let editingIndex: number = -1;
  
  $: computedEmptyMessage = emptyMessage || `No ${label.toLowerCase()} added yet`;

  function addItem() {
    if (selectedOptionId) {
      // Check for duplicates only if allowDuplicates is false
      if (!allowDuplicates) {
        const isDuplicate = items.some(item => {
          const itemId = itemDisplayComponent.getId(item);
          return itemId === selectedOptionId;
        });
        
        if (isDuplicate) {
          return; // Don't add duplicates
        }
      }
      
      // Call the parent's onAdd callback with the option ID
      onAdd(selectedOptionId);
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
    if (value) {
      addItem();
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
    
    {#if allowCreateNew && onCreateNew}
      <button
        type="button"
        on:click={() => onCreateNew()}
        class="flex items-center gap-2 px-3 py-2 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors flex-shrink-0"
      >
        <Plus size={16} />
        {createNewLabel}
      </button>
    {/if}
  </div>
  
  <!-- Add Item Section with RichSelect -->
  {#if !allowCreateNew}
    <div class="mb-3 mt-4">
      <div class="flex gap-2">
        <!-- RichSelect Component -->
        <div class="flex-1">
          <RichSelect
            selectedValue={selectedOptionId}
            {showSearch}
            {options}
            {placeholder}
            onSelect={handleSelectChange}
            displayComponent={optionDisplayComponent}
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
          class="flex items-center gap-3 p-3 rounded-lg border transition-all {
            allowItemEdit(item) ? 'cursor-pointer' : ''
          } {
            isHighlighted 
              ? 'bg-blue-900/20 border-blue-500 hover:bg-blue-900/30 hover:border-blue-400' 
              : 'bg-gray-700/20 border-gray-600 hover:bg-gray-700/30 hover:border-gray-500'
          }"
          on:click={() => allowItemEdit(item) && !itemDisplayComponent.supportsInlineEdit && onEdit(item, index)}
          tabindex={allowItemEdit(item) ? 0 : -1}
          role={allowItemEdit(item) ? 'button' : undefined}
        >
          
          <!-- Use slot if provided, otherwise check for inline editing -->
          <slot name="item" {item} {index}>
            {#if editingIndex === index && itemDisplayComponent.supportsInlineEdit && itemDisplayComponent.renderInlineEdit}
              {@const inlineEditConfig = itemDisplayComponent.renderInlineEdit(item, (updates) => {
                const updatedItem = { ...item, ...updates };
                onEdit(updatedItem, index)
              })}
              <svelte:component 
                this={inlineEditConfig.component} 
                {...inlineEditConfig.props} 
              />
            {:else}
              <ListSelectItem
                {item}
                displayComponent={itemDisplayComponent} />
            {/if}
          </slot>
          
          <!-- Action Buttons -->
          <div class="flex items-center gap-1">
            {#if allowItemEdit(item)}
              {#if itemDisplayComponent.supportsInlineEdit}
                <button
                  type="button"
                  on:click|stopPropagation={() => {
                    editingIndex = editingIndex === index ? -1 : index;
                  }}
                  class="p-1 text-gray-400 hover:text-white transition-colors"
                  title={editingIndex === index ? "Done editing" : "Edit"}
                >
                  <Edit size={16} />
                </button>
              {:else}
                <button
                  type="button"
                  on:click|stopPropagation={() => onEdit(item, index)}
                  class="p-1 text-gray-400 hover:text-white transition-colors"
                  title="Edit"
                >
                  <Edit size={16} />
                </button>
              {/if}
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