<!-- src/lib/shared/components/forms/DynamicField.svelte (Updated version) -->
<script lang="ts">
  import { AlertCircle, Server } from 'lucide-svelte';
  import type { ConfigField } from "$lib/shared/components/forms/types";
  import { createStyle } from '$lib/shared/utils/styling';
	import RichSelect from '$lib/shared/components/forms/RichSelect.svelte';
  
  export let field: ConfigField;
  export let value: any;
  export let error: string | null = null;
  export let onUpdate: (value: any) => void;
  export let disabled: boolean = false;  // NEW: disabled support
  
  // Use default value if no value is provided
  function getInitialValue() {
    // If a value is explicitly provided, use it
    if (value !== undefined && value !== null && value !== '') {
      return value;
    }
    
    // Otherwise, use the field's default value
    if (field.default_value !== undefined && field.default_value !== null) {
      return field.default_value;
    }
    
    // Fallback to empty string or appropriate default based on field type
    switch (field.field_type.base_type) {
      case 'boolean':
        return false;
      case 'integer':
        return 0;
      case 'select':
      case 'rich_select':
        return '';
      default:
        return '';
    }
  }
  
  // Initialize with the determined initial value
  let inputValue = getInitialValue();
  
  // Update internal value when prop changes, but preserve defaults
  $: {
    const newInitialValue = getInitialValue();
    if (newInitialValue !== inputValue) {
      inputValue = newInitialValue;
      // Notify parent of the default value if it wasn't already set
      if ((value === undefined || value === null || value === '') && 
          field.default_value !== undefined && field.default_value !== null) {
        onUpdate(field.default_value);
      }
    }
  }
  
  function handleInput(event: Event) {
    if (disabled) return; // Prevent input when disabled
    
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
  
  function handleRichSelectChange(value: any) {
    if (disabled) return; // Prevent input when disabled
    
    // Update internal value immediately
    inputValue = value;
    
    // Call parent update
    onUpdate(value);
  }

  // Disabled styling classes
  $: disabledClass = disabled ? 'opacity-50 cursor-not-allowed' : '';
  $: inputDisabledClass = disabled ? 'bg-gray-800 cursor-not-allowed' : 'bg-gray-700';
</script>

{#key field.id}
  <div class="space-y-2 {disabledClass}">
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
        {disabled}
        on:input={handleInput}
        class="w-full px-3 py-2 {inputDisabledClass} border border-gray-600 rounded-md text-white 
               focus:outline-none focus:ring-2 focus:ring-blue-500 
               {error ? 'border-red-500' : ''}
               {disabled ? 'text-gray-400' : ''}"
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
        {disabled}
        on:input={handleInput}
        class="w-full px-3 py-2 {inputDisabledClass} border border-gray-600 rounded-md text-white 
               focus:outline-none focus:ring-2 focus:ring-blue-500
               {error ? 'border-red-500' : ''}
               {disabled ? 'text-gray-400' : ''}"
      />
      
    {:else if field.field_type.base_type === 'boolean'}
      <label class="flex items-center gap-3 cursor-pointer {disabled ? 'cursor-not-allowed' : ''}">
        <input
          id={field.id}
          type="checkbox"
          checked={!!inputValue}
          {disabled}
          on:change={handleInput}
          class="rounded {inputDisabledClass} border-gray-600 text-blue-600 focus:ring-blue-500
                 {disabled ? 'cursor-not-allowed' : ''}"
        />
        <span class="text-sm {disabled ? 'text-gray-500' : 'text-gray-300'}">
          {field.help_text || 'Enable this option'}
        </span>
      </label>
      
    {:else if field.field_type.base_type === 'select'}
      <select
        id={field.id}
        value={inputValue}
        {disabled}
        on:change={handleInput}
        class="w-full px-3 py-2 {inputDisabledClass} border border-gray-600 rounded-md text-white 
               focus:outline-none focus:ring-2 focus:ring-blue-500
               {error ? 'border-red-500' : ''}
               {disabled ? 'text-gray-400 cursor-not-allowed' : ''}"
      >
        <option value="">{field.placeholder || 'Select an option...'}</option>
        {#each field.field_type.options || [] as option}
          <option value={option.value} disabled={option.disabled}>
            {option.label}
          </option>
        {/each}
      </select>
      
    {:else if field.field_type.base_type === 'rich_select'}
      <RichSelect
        selectedValue={inputValue}
        options={field.field_type.options?.map(opt => ({
          value: opt.value,
          label: opt.label,
          description: opt.description,
          disabled: opt.disabled || false,
          metadata: opt.metadata
        })) || []}
        placeholder={field.placeholder || 'Select an option...'}
        {disabled}
        onSelect={handleRichSelectChange}
        getOptionIcon={(opt) => {
          if (opt.metadata?.icon) {
            return createStyle(null, opt.metadata.icon).IconComponent;
          }
          return null;
        }}
        getOptionIconColor={(opt) => {
          if (opt.metadata?.color) {
            return createStyle(opt.metadata.color, null).colors.icon;
          }
          return 'text-gray-400';
        }}
      />
      
    {:else}
      <!-- Fallback for unknown field types -->
      <input
        id={field.id}
        type="text"
        value={inputValue}
        placeholder={field.placeholder}
        {disabled}
        on:input={handleInput}
        class="w-full px-3 py-2 {inputDisabledClass} border border-gray-600 rounded-md text-white 
               focus:outline-none focus:ring-2 focus:ring-blue-500
               {error ? 'border-red-500' : ''}
               {disabled ? 'text-gray-400' : ''}"
      />
    {/if}
    
    <!-- Help text -->
    {#if field.help_text && field.field_type.base_type !== 'boolean'}
      <p class="text-xs {disabled ? 'text-gray-500' : 'text-gray-400'}">
        {field.help_text}
      </p>
    {/if}
    
    <!-- Error message -->
    {#if error}
      <div class="flex items-center gap-2 text-red-400">
        <AlertCircle size={16} />
        <p class="text-xs">{error}</p>
      </div>
    {/if}
  </div>
{/key}