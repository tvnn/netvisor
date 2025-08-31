<script lang="ts">
  import { ChevronDown } from 'lucide-svelte';
	import type { SelectOption, RichSelectTag } from './types';
  
  export let label: string = '';
  export let selectedValue: string | null = '';
  export let options: SelectOption[] = [];
  export let placeholder: string = 'Select an option...';
  export let required: boolean = false;
  export let disabled: boolean = false;
  export let error: string | null = null;
  export let onSelect: (value: string) => void;
  
  // Optional props for customizing how options are rendered
  export let getOptionIcon: ((option: SelectOption) => any) | null = null;
  export let getOptionIconColor: ((option: SelectOption) => string) | null = null;
  export let getOptionBadge: ((option: SelectOption) => string | null) | null = null;
  export let getOptionBadgeColor: ((option: SelectOption) => string) | null = null;
  export let getOptionTag: ((option: SelectOption) => RichSelectTag | null) | null = null;
  export let getOptionStatusText: ((option: SelectOption) => string | null) | null = null;
  export let showDescriptionInClosedDropdown: boolean = false;
  export let showDescriptionUnderDropdown: boolean = false;
  
  let isOpen = false;
  let dropdownElement: HTMLDivElement;
  
  $: selectedOption = options.find(opt => opt.value === selectedValue);
  
  function handleSelect(value: string) {
    const option = options.find(opt => opt.value === value);
    if (option && !option.disabled) {
      onSelect(value);
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
      on:click={() => !disabled && (isOpen = !isOpen)}
      class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white 
             focus:outline-none focus:ring-2 focus:ring-blue-500 flex items-center justify-between
             {error ? 'border-red-500' : ''}
             {disabled ? 'opacity-50 cursor-not-allowed' : ''}"
      {disabled}
    >
      <div class="flex items-center gap-3 flex-1 min-w-0">
        {#if selectedOption}
          <!-- Icon -->
          {#if getOptionIcon}
            {@const icon = getOptionIcon(selectedOption)}
            {#if icon}
              <div class="w-6 h-6 rounded bg-gray-600 flex items-center justify-center flex-shrink-0">
                <svelte:component 
                  this={icon} 
                  class="w-3 h-3 {getOptionIconColor ? getOptionIconColor(selectedOption) : 'text-gray-300'}" 
                />
              </div>
            {/if}
          {/if}
          
          <!-- Label and description -->
          <div class="flex-1 min-w-0 text-left">
            <span class="block truncate">{selectedOption.label}</span>
            {#if showDescriptionInClosedDropdown && selectedOption.description}
              <span class="block text-xs text-gray-400 truncate">{selectedOption.description}</span>
            {/if}
          </div>
          
          <!-- Tag -->
          {#if getOptionTag}
            {@const tag = getOptionTag(selectedOption)}
            {#if tag}
              <span class="inline-block px-2 py-0.5 text-xs rounded flex-shrink-0 {tag.textColor} {tag.bgColor}">
                {tag.text}
              </span>
            {/if}
          {/if}
        {:else}
          <span class="text-gray-400">{placeholder}</span>
        {/if}
      </div>
      
      <ChevronDown class="w-4 h-4 text-gray-400 transition-transform flex-shrink-0 {isOpen ? 'rotate-180' : ''}" />
    </button>
    
    <!-- Description below trigger (optional) -->
    {#if selectedOption && selectedOption.description && showDescriptionUnderDropdown}
      <div class="mt-2">
        <p class="text-sm text-gray-400">
          {selectedOption.description}
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
          {@const tag = getOptionTag ? getOptionTag(option) : null}
          
          <button
            type="button"
            on:click={() => handleSelect(option.value)}
            class="w-full px-3 py-3 text-left transition-colors border-b border-gray-600 last:border-b-0
                   {option.disabled ? 'cursor-not-allowed opacity-50' : 'hover:bg-gray-600'}"
            disabled={option.disabled}
          >
            <div class="flex items-start gap-3">
              <!-- Icon -->
              {#if getOptionIcon}
                {@const icon = getOptionIcon(option)}
                {#if icon}
                  <div class="w-8 h-8 rounded-lg bg-gray-600 flex items-center justify-center mt-0.5 flex-shrink-0">
                    <svelte:component 
                      this={icon} 
                      class="w-4 h-4 {getOptionIconColor ? getOptionIconColor(option) : 'text-gray-300'}" 
                    />
                  </div>
                {/if}
              {/if}
              
              <!-- Content -->
              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 flex-wrap">
                  <h4 class="font-medium text-white">{option.label}</h4>
                  
                  <!-- Badge -->
                  {#if getOptionBadge}
                    {@const badge = getOptionBadge(option)}
                    {#if badge}
                      <span class="inline-block px-2 py-1 text-xs rounded
                                  {getOptionBadgeColor ? getOptionBadgeColor(option) : 'bg-gray-600 text-gray-300'}">
                        {badge}
                      </span>
                    {/if}
                  {/if}
                  
                  <!-- Tag -->
                  {#if tag}
                    <span class="inline-block px-2 py-1 text-xs rounded {tag.textColor} {tag.bgColor}">
                      {tag.text}
                    </span>
                  {/if}
                </div>
                
                <!-- Description -->
                {#if option.description}
                  <p class="text-sm text-gray-400 mt-1 line-clamp-2">{option.description}</p>
                {/if}
                
                <!-- Status Text -->
                {#if getOptionStatusText}
                  {@const statusText = getOptionStatusText(option)}
                  {#if statusText}
                    <p class="text-xs mt-1 text-gray-400">
                      {statusText}
                    </p>
                  {/if}
                {/if}
              </div>
            </div>
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>