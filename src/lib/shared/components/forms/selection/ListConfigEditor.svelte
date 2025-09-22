<script lang="ts" generics="TItem">
  // Core data
  export let items: TItem[] = [];
  
  // Layout configuration  
  export let listPanelWidth: string = 'w-2/5';
  export let configPanelWidth: string = 'w-3/5';
  export let loading: boolean = false;
  
  // Event handlers
  export let onChange: (item: TItem, index: number) => void = () => {};
  export let onItemAdded: (newIndex: number) => void = (newIndex) => {
    selectedIndex = newIndex;
  };
  
  // Internal state
  let selectedIndex: number = -1;
  
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
        <slot 
          name="list" 
          {items}
          {selectedIndex}
          onEdit={handleEdit}
          highlightedIndex={selectedIndex}
        >
          <!-- Default slot content if no list slot provided -->
          <div class="text-gray-400">No list component provided</div>
        </slot>
      </div>
      
      <!-- Spacer to make left panel take full height -->
      <div class="flex-1"></div>
    </div>

    <!-- Right Panel - Configuration -->
    <div class="{configPanelWidth} border-l border-gray-600 p-6 overflow-y-auto">
      <slot 
        name="config" 
        {selectedItem} 
        {selectedIndex} 
        onChange={handleItemChange}
      >
        <div class="flex items-center justify-center h-32 text-gray-400">
          <p>Select an item to configure</p>
        </div>
      </slot>
    </div>
  </div>
{/if}