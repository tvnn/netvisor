<script lang="ts" generics="TItem, TOption">
  import ListManager from './ListManager.svelte';
  import type { TagProps } from '../data/types';
  
  // Core data
  export let items: TItem[] = [];
  export let options: TOption[] = [];
  export let form: any;
  
  // Labels and help
  export let label: string;
  export let helpText: string = '';
  export let emptyMessage: string = '';
  export let placeholder: string = 'Select an item to add';
  
  // Layout configuration  
  export let listPanelWidth: string = 'w-2/5';
  export let configPanelWidth: string = 'w-3/5';
  export let loading: boolean = false;
  
  // ListManager behavior
  export let allowDuplicates: boolean = false;
  export let allowReorder: boolean = false;
  export let allowDirectAdd: boolean = true;
  export let allowItemRemove: (item: TItem) => boolean = () => true;
  export let allowItemEdit: (item: TItem) => boolean = () => true;
  
  // Display functions for options (dropdown)
  export let getOptionId: (option: TOption) => string;
  export let getOptionLabel: (option: TOption) => string;
  export let getOptionDescription: (option: TOption) => string = () => '';
  export let getOptionIcon: (option: TOption) => any = () => null;
  export let getOptionIconColor: (option: TOption) => string = () => '';
  export let getOptionTags: (option: TOption) => TagProps[] = () => [];
  export let getOptionIsDisabled: (option: TOption) => boolean = () => false;
  
  // Display functions for items (list)
  export let getItemId: (item: TItem) => string;
  export let getItemLabel: (item: TItem) => string;
  export let getItemDescription: (item: TItem) => string = () => '';
  export let getItemIcon: (item: TItem) => any = () => null;
  export let getItemIconColor: (item: TItem) => string = () => '';
  export let getItemTags: (item: TItem) => TagProps[] = () => [];
  
  // Event handlers
  export let onAdd: (optionId: string) => void;
  export let onRemove: (index: number) => void = (index) => {
    items = items.filter((_, i) => i !== index);
    if (selectedIndex === index) selectedIndex = -1;
    else if (selectedIndex > index) selectedIndex--;
  };
  export let onChange: (item: TItem, index: number) => void = () => {};
  export let onItemAdded: (newIndex: number) => void = (newIndex) => {
    selectedIndex = newIndex;
  };
  
  // Internal state
  let selectedIndex: number = -1;
  let selectedOptionId = '';
  
  // Computed values for slot consumers
  $: selectedItem = selectedIndex >= 0 ? items[selectedIndex] : null;
  
  // Track previous items length to detect when items are added
  let previousItemsLength = 0;
  $: {
    if (items.length > previousItemsLength) {
      // Items were added, select the last one
      onItemAdded(items.length - 1);
    } else if (items.length === 1 && selectedIndex === -1) {
      // Auto-select the first (and only) item when there's exactly one item
      selectedIndex = 0;
    } else if (items.length === 0) {
      // Clear selection when no items
      selectedIndex = -1;
    }
    previousItemsLength = items.length;
  }
  
  // Event handlers
  function handleEdit(item: TItem, index: number) {
    selectedIndex = index;
  }
  
  function handleItemChange(updatedItem: TItem) {
    if (selectedIndex >= 0 && selectedIndex < items.length) {
      items[selectedIndex] = updatedItem;
      items = items; // Trigger reactivity
      onChange(updatedItem, selectedIndex);
    }
  }
  
  function handleAdd(optionId: string) {
    console.log('ListConfigEditor handleAdd called with:', optionId);
    onAdd(optionId);
    selectedOptionId = ''; // Clear the selection
    console.log('After onAdd, items length:', items.length);
  }
  
  function handleRemove(index: number) {
    onRemove(index);
  }
</script>

{#if loading}
  <div class="h-full flex items-center justify-center">
    <div class="flex items-center gap-3 text-gray-400">
      <div class="w-5 h-5 border-2 border-gray-400 border-t-transparent rounded-full animate-spin"></div>
      Loading...
    </div>
  </div>
{:else}
  <div class="h-full flex gap-6">
    <!-- Left Panel - List Manager (Full Height) -->
    <div class="{listPanelWidth} flex flex-col relative">
      <div class="p-6">
        <ListManager
          {label}
          {helpText}
          {items}
          options={options}
          {allowDuplicates}
          {allowReorder}
          {allowDirectAdd}
          {allowItemRemove}
          {allowItemEdit}
          placeholder={placeholder}
          {emptyMessage}
          
          getOptionId={getOptionId}
          getOptionLabel={getOptionLabel}
          getOptionDescription={getOptionDescription}
          getOptionIcon={getOptionIcon}
          getOptionIconColor={getOptionIconColor}
          getOptionTags={getOptionTags}
          getOptionIsDisabled={getOptionIsDisabled}
          
          getItemId={getItemId}
          getItemLabel={getItemLabel}
          getItemDescription={getItemDescription}
          getItemIcon={getItemIcon}
          getItemIconColor={getItemIconColor}
          getItemTags={getItemTags}
          
          onEdit={handleEdit}
          onAdd={handleAdd}
          onRemove={handleRemove}
          highlightedIndex={selectedIndex}
        />
      </div>
      
      <!-- Spacer to make left panel take full height -->
      <div class="flex-1"></div>
    </div>

    <!-- Right Panel - Configuration -->
    <div class="{configPanelWidth} border-l border-gray-600 p-6 overflow-y-auto">
      <slot 
        name="config" 
        selectedItem={selectedItem} 
        selectedIndex={selectedIndex} 
        onChange={handleItemChange}
      >
        <div class="flex items-center justify-center h-32 text-gray-400">
          <p>Select an item to configure</p>
        </div>
      </slot>
    </div>
  </div>
{/if}