<script lang="ts">
  import type { Writable } from 'svelte/store';
	import FormField, { getInputClasses } from './FormField.svelte';
	import type { FieldType, FormApi } from '../types';
  
  export let label: string;
  export let formApi: FormApi;
  export let field: FieldType;
  export let id: string;
  export let placeholder: string = '';
  export let required: boolean = false;
  export let helpText: string = '';
  export let type: 'text' | 'email' | 'password' = 'text';
  export let disabled: boolean = false;
  export let showValidation: boolean = true;
  
  // Enable validation on user interaction
  function enableValidation() {
    showValidation = true;
    if ($field) $field.valid;
  }
</script>

<FormField 
  {label} 
  {formApi}
  {field}
  {required} 
  {helpText} 
  errors={showValidation ? $field.errors : []} 
  {showValidation} 
  {id}
>
  <input
    id={id}
    {type}
    bind:value={$field.value}
    {placeholder}
    {disabled}
    class={getInputClasses(showValidation && $field.errors.length > 0)}
    on:blur={enableValidation}
    on:input={() => showValidation && field.validate()}
  />
</FormField>