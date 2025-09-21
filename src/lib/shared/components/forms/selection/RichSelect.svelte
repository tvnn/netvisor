<script lang="ts" generics="T">
  import { ChevronDown } from 'lucide-svelte';
  import ListSelectItem from './ListSelectItem.svelte';
  import type { EntityDisplayComponent } from './types';
  import { tick } from 'svelte';
  
  export let label: string = '';
  export let selectedValue: string | null = '';
  export let options: T[] = [];
  export let placeholder: string = 'Select an option...';
  export let required: boolean = false;
  export let disabled: boolean = false;
  export let error: string | null = null;
  export let onSelect: (value: string) => void;
  export let showDescriptionUnderDropdown: boolean = false;
  export let displayComponent: EntityDisplayComponent<T>;
  
  let isOpen = false;
  let dropdownElement: HTMLDivElement;
  let triggerElement: HTMLButtonElement;
  let dropdownPosition = { top: 0, left: 0, width: 0 };
  let openUpward = false; // Track direction separately
  let scrollableParents: Element[] = [];
  
  // Find all scrollable parent elements
  function findScrollableParents(element: Element): Element[] {
    const parents: Element[] = [];
    let parent = element.parentElement;
    
    while (parent && parent !== document.body) {
      const overflow = window.getComputedStyle(parent).overflow;
      if (overflow === 'auto' || overflow === 'scroll' || overflow === 'hidden') {
        parents.push(parent);
      }
      parent = parent.parentElement;
    }
    
    return parents;
  }
  
  // Add scroll listeners to all scrollable parents
  function addScrollListeners() {
    if (!triggerElement) return;
    
    scrollableParents = findScrollableParents(triggerElement);
    scrollableParents.forEach(parent => {
      parent.addEventListener('scroll', handleScroll, { passive: true });
    });
  }
  
  // Remove scroll listeners
  function removeScrollListeners() {
    scrollableParents.forEach(parent => {
      parent.removeEventListener('scroll', handleScroll);
    });
    scrollableParents = [];
  }
  
  $: selectedItem = options.find(i => displayComponent.getId(i) === selectedValue);
  
  // Group options by category when getCategory is provided
  $: groupedOptions = (() => {
    if (!displayComponent.getCategory) {
      return [{ category: null, options: options }];
    }
    
    const groups = new Map<string | null, T[]>();
    
    options.forEach(option => {
      const category = displayComponent.getCategory!(option);
      if (!groups.has(category)) {
        groups.set(category, []);
      }
      groups.get(category)!.push(option);
    });
    
    // Sort categories alphabetically, with null category first
    const sortedEntries = Array.from(groups.entries()).sort(([a], [b]) => {
      if (a === null) return -1;
      if (b === null) return 1;
      return a.localeCompare(b);
    });
    
    return sortedEntries.map(([category, options]) => ({ category, options }));
  })();
  
  async function updateDropdownPosition() {
    if (!triggerElement) return;
    
    await tick();
    const rect = triggerElement.getBoundingClientRect();
    const viewportHeight = window.innerHeight;
    const gap = 4;
    
    // Calculate available space above and below
    const spaceBelow = viewportHeight - rect.bottom;
    const spaceAbove = rect.top;
    
    // Determine if we should open upward
    // Only switch to upward if there's significantly more space above
    const minSpaceNeeded = 200;
    openUpward = spaceBelow < minSpaceNeeded && spaceAbove > spaceBelow + 100;
    
    dropdownPosition = {
      top: openUpward 
        ? rect.top - gap // Position above trigger, will use transform to move it up
        : rect.bottom + gap, // Position below trigger
      left: rect.left,
      width: rect.width
    };
  }
  
  async function handleToggle(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();
    if (!disabled) {
      if (!isOpen) {
        isOpen = true;
        await updateDropdownPosition();
        addScrollListeners();
      } else {
        isOpen = false;
        removeScrollListeners();
      }
    }
  }
  
  function handleSelect(value: string) {
    try {
      const item = options.find(i => displayComponent.getId(i) === value);
      if (item && !displayComponent.getIsDisabled?.(item)) {
        isOpen = false;  // Close dropdown first
        removeScrollListeners(); // Clean up listeners
        onSelect(value); // Then call the handler
      }
    } catch (e) {
      console.warn('Error in handleSelect:', e);
      isOpen = false;
      removeScrollListeners();
    }
  }
  
  function handleClickOutside(event: MouseEvent) {
    if (dropdownElement && !dropdownElement.contains(event.target as Node) &&
        triggerElement && !triggerElement.contains(event.target as Node)) {
      isOpen = false;
      removeScrollListeners();
    }
  }
  
  // Update position on scroll/resize
  function handleWindowChange() {
    if (isOpen) {
      updateDropdownPosition();
    }
  }
  
  // Handle scroll events specifically - need to update position immediately
  function handleScroll() {
    if (isOpen) {
      updateDropdownPosition();
    }
  }
  
  // Clean up listeners on component destroy
  import { onDestroy } from 'svelte';
  
  onDestroy(() => {
    removeScrollListeners();
  });
</script>

<svelte:window 
  on:click={handleClickOutside} 
  on:scroll={handleScroll}
  on:resize={handleWindowChange}
/>

<div class="">
  <!-- Label -->
  {#if label}
    <div class="block text-sm font-medium text-gray-300">
      {label}
      {#if required}
        <span class="text-red-400 ml-1">*</span>
      {/if}
    </div>
  {/if}
  
  <!-- Dropdown Container -->
  <div class="relative">
    <!-- Dropdown Trigger -->
    <button
      bind:this={triggerElement}
      type="button"
      on:click={handleToggle}
      class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white 
             focus:outline-none focus:ring-2 focus:ring-blue-500 flex items-center justify-between
             {error ? 'border-red-500' : ''}
             {disabled ? 'opacity-50 cursor-not-allowed' : ''}"
      {disabled}
    >
      <div class="flex items-center gap-3 flex-1 min-w-0">
        {#if selectedItem}
          <ListSelectItem
            item={selectedItem}
            {displayComponent} />
        {:else}
          <span class="text-gray-400">{placeholder}</span>
        {/if}
      </div>
      
      <ChevronDown class="w-4 h-4 text-gray-400 transition-transform flex-shrink-0 {isOpen ? 'rotate-180' : ''}" />
    </button>
    
    <!-- Description below trigger (optional) -->
    {#if selectedItem && displayComponent.getDescription?.(selectedItem) && showDescriptionUnderDropdown}
      <div class="mt-2">
        <p class="text-sm text-gray-400">
          {displayComponent.getDescription(selectedItem)}
        </p>
      </div>
    {/if}
    
    <!-- Error Message -->
    {#if error}
      <div class="flex items-center gap-2 text-red-400 text-sm mt-1">
        <span>{error}</span>
      </div>
    {/if}
  </div>
</div>

<!-- Portal dropdown to document.body -->
{#if isOpen && !disabled}
  <div 
    bind:this={dropdownElement}
    class="fixed z-[9999] bg-gray-700 border border-gray-600 rounded-md shadow-lg max-h-96 overflow-y-auto"
    style="top: {dropdownPosition.top}px; left: {dropdownPosition.left}px; width: {dropdownPosition.width}px;
           {openUpward ? 'transform: translateY(-100%);' : ''}"
  >
    {#each groupedOptions as group, groupIndex}
      <!-- Category Header -->
      {#if group.category !== null}
        <div class="px-3 py-2 text-xs font-semibold text-gray-400 uppercase tracking-wide bg-gray-800 border-b border-gray-600 sticky top-0">
          {group.category}
        </div>
      {/if}
      
      <!-- Options in this category -->
      {#each group.options as option, optionIndex}
        {@const isLastInGroup = optionIndex === group.options.length - 1}
        {@const isLastGroup = groupIndex === groupedOptions.length - 1}
        <button
          type="button"
          on:click={(e) => {
            e.preventDefault();
            e.stopPropagation();
            if (!displayComponent.getIsDisabled?.(option)) {
              handleSelect(displayComponent.getId(option));
            }
          }}
          class="w-full px-3 py-3 text-left transition-colors 
                 {!isLastInGroup || !isLastGroup ? 'border-b border-gray-600' : ''}
                 {displayComponent.getIsDisabled?.(option) ? 'cursor-not-allowed opacity-50' : 'hover:bg-gray-600'}"
          disabled={displayComponent.getIsDisabled?.(option)}
        >
          <ListSelectItem
            item={option}
            {displayComponent} />
        </button>
      {/each}
    {/each}
  </div>
{/if}