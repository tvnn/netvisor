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
  let openUpward = false;
  
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
  
  // Simple one-time positioning when dropdown opens
  async function calculatePosition() {
    if (!triggerElement) return;
    
    await tick();
    const rect = triggerElement.getBoundingClientRect();
    const viewportHeight = window.innerHeight;
    const dropdownHeight = 384; // max-h-96 = 24rem = 384px
    const gap = 1; // Minimal gap to prevent overlap
    
    // Simple logic: if not enough space below, open upward
    const spaceBelow = viewportHeight - rect.bottom - gap;
    openUpward = spaceBelow < dropdownHeight && rect.top > spaceBelow;
    
    dropdownPosition = {
      top: openUpward ? rect.top - gap : rect.bottom + gap,
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
        await calculatePosition(); // Calculate once when opening
      } else {
        isOpen = false;
      }
    }
  }
  
  function handleSelect(value: string) {
    try {
      const item = options.find(i => displayComponent.getId(i) === value);
      if (item && !displayComponent.getIsDisabled?.(item)) {
        isOpen = false;
        onSelect(value);
      }
    } catch (e) {
      console.warn('Error in handleSelect:', e);
      isOpen = false;
    }
  }
  
  function handleClickOutside(event: MouseEvent) {
    if (dropdownElement && !dropdownElement.contains(event.target as Node) &&
        triggerElement && !triggerElement.contains(event.target as Node)) {
      isOpen = false;
    }
  }
</script>

<!-- Only handle outside clicks -->
<svelte:window on:click={handleClickOutside} />

<div class="relative">
  <!-- Label -->
  {#if label}
    <div class="block text-sm font-medium text-gray-300">
      {label}
      {#if required}
        <span class="text-red-400 ml-1">*</span>
      {/if}
    </div>
  {/if}
  
  <!-- Dropdown Trigger -->
  <button
    bind:this={triggerElement}
    type="button"
    on:click={handleToggle}
    class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white 
           focus:outline-none focus:ring-2 focus:ring-blue-500 flex items-center justify-between
           {error ? 'border-red-500' : ''}
           {disabled || options.length == 0 ? 'opacity-50 cursor-not-allowed' : ''}"
    disabled={disabled || options.length == 0}
  >
    <div class="flex items-center gap-3 flex-1 min-w-0">
      {#if selectedItem}
        <ListSelectItem
          item={selectedItem}
          {displayComponent} />
      {:else}
        <span class="text-gray-400">{options.length == 0 ? 'No options available' : placeholder}</span>
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

<!-- Portal dropdown - positioned once, no scroll tracking -->
{#if isOpen && !disabled}
  <div 
    bind:this={dropdownElement}
    class="fixed z-[9999] bg-gray-700 border border-gray-600 rounded-md shadow-lg max-h-96 overflow-y-auto scroll-smooth"
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