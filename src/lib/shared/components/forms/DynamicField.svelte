<!-- src/lib/shared/components/forms/DynamicFieldSvelteForms.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { AlertCircle } from 'lucide-svelte';
  import { field as svelteFormsField } from 'svelte-forms';
  import { required } from 'svelte-forms/validators';
  import type { ConfigField } from './types';
  import { createStyle } from '$lib/shared/utils/styling';
  import RichSelect from '$lib/shared/components/forms/RichSelect.svelte';
  import { maxLength, portRange } from './validators';
  
  export let form: any;
  export let field: ConfigField;
  export let fieldId: string;
  export let value: any;
  export let onUpdate: (value: any) => void;
  export let disabled: boolean = false;
  
  let formField: any;
  
  // Create form field with appropriate validators
  function createFormField() {
    const validators = [];
    
    // Add required validator if needed
    if (field.required) {
      validators.push(required());
    }
    
    // Add type-specific validators
    if (field.field_type.base_type === 'string' && field.field_type.constraints?.maxLength) {
      validators.push(maxLength(field.field_type.constraints.maxLength));
    }
    
    if (field.field_type.base_type === 'integer') {
      validators.push(portRange()); // Assuming integer fields are ports, customize as needed
    }
    
    return svelteFormsField(fieldId, getInitialValue(), validators);
  }
  
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
        return '';
      case 'select':
      case 'rich_select':
        return '';
      default:
        return '';
    }
  }
  
  // Initialize form field
  onMount(() => {
    formField = createFormField();
    if (form) {
      form[fieldId] = formField
    }
  });
  
  // Update parent when field value changes
  $: if (formField && $formField) {
    let processedValue = $formField.value;
    
    // Type conversion based on field type
    if (field.field_type.base_type === 'integer') {
      if (processedValue === '') {
        processedValue = '';
      } else {
        const parsed = parseInt(processedValue);
        processedValue = isNaN(parsed) ? '' : parsed;
      }
    } else if (field.field_type.base_type === 'boolean') {
      processedValue = Boolean(processedValue);
    }
    
    onUpdate(processedValue);
  }
  
  // Update field when external value changes
  $: if (formField && value !== $formField.value && value !== undefined) {
    formField.set(value);
  }
  
  function handleRichSelectChange(newValue: any) {
    if (disabled || !formField) return;
    formField.set(newValue);
  }

  // Disabled styling classes
  $: disabledClass = disabled ? 'opacity-50 cursor-not-allowed' : '';
  $: inputDisabledClass = disabled ? 'bg-gray-800 cursor-not-allowed' : 'bg-gray-700';
  $: errorClass = formField && $formField.errors.length > 0 ? 'border-red-500' : 'border-gray-600';
</script>

{#if formField}
  <div class="space-y-2 {disabledClass}">
    <label for={fieldId} class="block text-sm font-medium text-gray-300">
      {field.label}
      {#if field.required}
        <span class="text-red-400 ml-1">*</span>
      {/if}
    </label>
    
    {#if field.field_type.base_type === 'string'}
      <input
        id={fieldId}
        type="text"
        bind:value={$formField.value}
        placeholder={field.placeholder}
        {disabled}
        class="w-full px-3 py-2 {inputDisabledClass} border {errorClass} rounded-md text-white 
               focus:outline-none focus:ring-2 focus:ring-blue-500 
               {disabled ? 'text-gray-400' : ''}"
      />
      
    {:else if field.field_type.base_type === 'integer'}
      <input
        id={fieldId}
        type="number"
        bind:value={$formField.value}
        placeholder={field.placeholder}
        min={field.field_type.constraints?.min}
        max={field.field_type.constraints?.max}
        step={field.field_type.constraints?.step || 1}
        {disabled}
        class="w-full px-3 py-2 {inputDisabledClass} border {errorClass} rounded-md text-white 
               focus:outline-none focus:ring-2 focus:ring-blue-500
               {disabled ? 'text-gray-400' : ''}"
      />
      
    {:else if field.field_type.base_type === 'boolean'}
      <label class="flex items-center gap-3 cursor-pointer {disabled ? 'cursor-not-allowed' : ''}">
        <input
          id={fieldId}
          type="checkbox"
          bind:checked={$formField.value}
          {disabled}
          class="rounded {inputDisabledClass} border-gray-600 text-blue-600 focus:ring-blue-500
                 {disabled ? 'cursor-not-allowed' : ''}"
        />
        <span class="text-sm {disabled ? 'text-gray-500' : 'text-gray-300'}">
          {field.help_text || 'Enable this option'}
        </span>
      </label>
      
    {:else if field.field_type.base_type === 'select'}
      <select
        id={fieldId}
        bind:value={$formField.value}
        {disabled}
        class="w-full px-3 py-2 {inputDisabledClass} border {errorClass} rounded-md text-white 
               focus:outline-none focus:ring-2 focus:ring-blue-500
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
        selectedValue={$formField.value}
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
        id={fieldId}
        type="text"
        bind:value={$formField.value}
        placeholder={field.placeholder}
        {disabled}
        class="w-full px-3 py-2 {inputDisabledClass} border {errorClass} rounded-md text-white 
               focus:outline-none focus:ring-2 focus:ring-blue-500
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
    {#if $formField.errors.length > 0}
      <div class="flex items-center gap-2 text-red-400">
        <AlertCircle size={16} />
        <p class="text-xs">{$formField.errors[0]}</p>
      </div>
    {/if}
  </div>
{/if}