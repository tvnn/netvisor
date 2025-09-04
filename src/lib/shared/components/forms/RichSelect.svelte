<script lang="ts">
  import { ChevronDown } from 'lucide-svelte';
	import Tag from '../data/Tag.svelte';
	import ListSelectItem from './ListSelectItem.svelte';
	import type { TagProps } from '../data/types';
  
  export let label: string = '';
  export let selectedValue: string | null = '';
  export let options: any[] = [];
  export let placeholder: string = 'Select an option...';
  export let required: boolean = false;
  export let disabled: boolean = false;
  export let error: string | null = null;
  export let onSelect: (value: string) => void;
  export let getOptionId: (item: any) => string;

  export let getOptionIcon: (item: any) => any | null = (item) => null;
  export let getOptionIconColor: (item: any) => string | null = (item) => null;
  export let getOptionTags: (item: any) => TagProps[] = (item) => [];
  export let getOptionLabel: (item: any) => string | null = (item) => null
  export let getOptionDescription: (item: any) => string | null = (item) => null
  export let getOptionIsDisabled: (item: any) => boolean = (item) => false

  export let showDescriptionUnderDropdown: boolean = false;
  
  let isOpen = false;
  let dropdownElement: HTMLDivElement;
  
  $: selectedItem = options.find(i => getOptionId(i) === selectedValue);
  
  function handleSelect(value: string) {
    try {
      const item = options.find(i => getOptionId(i) === value);
      if (item && !getOptionIsDisabled(item)) {
        isOpen = false;  // Close dropdown first
        onSelect(value); // Then call the handler
      }
    } catch (e) {
      console.warn('Error in handleSelect:', e);
      isOpen = false;
    }
  }
  
  function handleClickOutside(event: MouseEvent) {
    if (dropdownElement && !dropdownElement.contains(event.target as Node)) {
      isOpen = false;
    }
  }
</script>

<svelte:window on:click={handleClickOutside} />

<div class="" bind:this={dropdownElement}>
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
      type="button"
      on:click={(e) => {
        e.preventDefault();
        e.stopPropagation();
        if (!disabled) {
          isOpen = !isOpen;
        }
      }}
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
            getIcon={getOptionIcon}
            getIconColor={getOptionIconColor}
            getTags={getOptionTags}
            getLabel={getOptionLabel}
            getDescription={getOptionDescription} />
        {:else}
          <span class="text-gray-400">{placeholder}</span>
        {/if}
      </div>
      
      <ChevronDown class="w-4 h-4 text-gray-400 transition-transform flex-shrink-0 {isOpen ? 'rotate-180' : ''}" />
    </button>
    
    <!-- Description below trigger (optional) -->
    {#if selectedItem && getOptionDescription(selectedItem) && showDescriptionUnderDropdown}
      <div class="mt-2">
        <p class="text-sm text-gray-400">
          {getOptionDescription(selectedItem)}
        </p>
      </div>
    {/if}
    
    <!-- Error Message -->
    {#if error}
      <div class="flex items-center gap-2 text-red-400 text-sm mt-1">
        <span>{error}</span>
      </div>
    {/if}
    
    <!-- Dropdown Menu -->
    {#if isOpen && !disabled}
      <div class="absolute z-50 w-full bg-gray-700 border border-gray-600 rounded-md shadow-lg max-h-96 overflow-y-auto mt-1">
        {#each options as option}          
          <button
            type="button"
            on:click={(e) => {
              e.preventDefault();
              e.stopPropagation();
              if (!getOptionIsDisabled(option)) {
                handleSelect(getOptionId(option));
              }
            }}
            class="w-full px-3 py-3 text-left transition-colors border-b border-gray-600 last:border-b-0
                   {getOptionIsDisabled(option) ? 'cursor-not-allowed opacity-50' : 'hover:bg-gray-600'}"
            disabled={getOptionIsDisabled(option)}
          >
            <ListSelectItem
              item={option}
              getIcon={getOptionIcon}
              getIconColor={getOptionIconColor}
              getTags={getOptionTags}
              getLabel={getOptionLabel}
              getDescription={getOptionDescription} />
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>