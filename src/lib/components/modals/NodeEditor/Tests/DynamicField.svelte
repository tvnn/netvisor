<script lang="ts">
  import { AlertCircle, Server } from 'lucide-svelte';
  import type { ConfigField } from '$lib/stores/schema';
  
  export let field: ConfigField;
  export let value: any;
  export let error: string | null = null;
  export let onUpdate: (value: any) => void;
  
  // Stable value to prevent unnecessary re-renders
  let inputValue = value || '';
  
  // Update internal value when prop changes
  $: if (value !== inputValue) {
    inputValue = value || '';
  }
  
  function handleInput(event: Event) {
    const target = event.target as HTMLInputElement | HTMLSelectElement;
    let newValue: any = target.value;
    
    // Type conversion based on field type
    if (field.field_type.base_type === 'integer') {
      newValue = parseInt(newValue) || 0;
    } else if (field.field_type.base_type === 'boolean') {
      newValue = (target as HTMLInputElement).checked;
    }
    
    // Update internal value immediately
    inputValue = newValue;
    
    // Call parent update
    onUpdate(newValue);
  }
</script>

{#key field.id}
  <div class="space-y-2">
    <label for={field.id} class="block text-sm font-medium text-gray-300">
      {field.label}
      {#if field.required}
        <span class="text-red-400 ml-1">*</span>
      {/if}
    </label>
    
    {#if field.field_type.base_type === 'string'}
      <input
        id={field.id}
        type="text"
        value={inputValue}
        placeholder={field.placeholder}
        on:input={handleInput}
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white 
               focus:outline-none focus:ring-2 focus:ring-blue-500 
               {error ? 'border-red-500' : ''}"
      />
      
    {:else if field.field_type.base_type === 'integer'}
      <input
        id={field.id}
        type="number"
        value={inputValue}
        placeholder={field.placeholder}
        min={field.field_type.constraints?.min}
        max={field.field_type.constraints?.max}
        step={field.field_type.constraints?.step || 1}
        on:input={handleInput}
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white 
               focus:outline-none focus:ring-2 focus:ring-blue-500
               {error ? 'border-red-500' : ''}"
      />
      
    {:else if field.field_type.base_type === 'boolean'}
      <label class="flex items-center gap-3 cursor-pointer">
        <input
          id={field.id}
          type="checkbox"
          checked={!!inputValue}
          on:change={handleInput}
          class="rounded bg-gray-700 border-gray-600 text-blue-600 focus:ring-blue-500"
        />
        <span class="text-sm text-gray-300">{field.help_text || 'Enable this option'}</span>
      </label>
      
    {:else if field.field_type.base_type === 'select'}
      <select
        id={field.id}
        value={inputValue}
        on:change={handleInput}
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white 
               focus:outline-none focus:ring-2 focus:ring-blue-500
               {error ? 'border-red-500' : ''}"
      >
        {#if !field.required}
          <option value="">Select an option...</option>
        {/if}
        {#each field.field_type.options || [] as option}
          <option value={option.value} disabled={option.disabled}>
            {option.label}
          </option>
        {/each}
      </select>
      
    {:else if field.field_type.base_type === 'node_selector'}
      <div class="space-y-3">
        {#if !field.field_type.options || field.field_type.options.length === 0}
          <div class="p-4 bg-yellow-900/20 border border-yellow-600 rounded-lg">
            <div class="flex items-center gap-2">
              <AlertCircle class="w-4 h-4 text-yellow-400" />
              <span class="text-sm text-yellow-200">No compatible nodes available</span>
            </div>
            <p class="text-xs text-yellow-300 mt-1">
              Create a node with the required capabilities first.
            </p>
          </div>
        {:else}
          <div class="grid grid-cols-1 gap-3">
            {#each field.field_type.options as nodeOption}
              <label class="flex items-start gap-3 p-3 bg-gray-700/50 border border-gray-600 rounded-lg 
                            cursor-pointer hover:bg-gray-700/70 transition-colors
                            {inputValue === nodeOption.value ? 'border-blue-500 bg-blue-900/20' : ''}">
                <input
                  type="radio"
                  name={field.id}
                  value={nodeOption.value}
                  checked={inputValue === nodeOption.value}
                  on:change={handleInput}
                  class="mt-0.5 text-blue-600 bg-gray-700 border-gray-600 focus:ring-blue-500"
                />
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2">
                    <Server class="w-4 h-4 text-blue-400" />
                    <span class="font-medium text-white">{nodeOption.label}</span>
                  </div>
                  {#if nodeOption.description}
                    <p class="text-sm text-gray-400 mt-1">{nodeOption.description}</p>
                  {/if}
                </div>
              </label>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
    
    <!-- Help Text -->
    {#if field.help_text && field.field_type.base_type !== 'boolean'}
      <p class="text-xs text-gray-400">{field.help_text}</p>
    {/if}
    
    <!-- Error Message -->
    {#if error}
      <p class="text-xs text-red-400 flex items-center gap-1">
        <AlertCircle class="w-3 h-3" />
        {error}
      </p>
    {/if}
  </div>
{/key}