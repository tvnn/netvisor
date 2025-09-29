<script lang="ts">
	import type { FieldType, FormApi } from '../types';
	import FormField from './FormField.svelte';
	import { getInputClasses } from './FormField.svelte';
	import type { Writable } from 'svelte/store';

	export let label: string;
	export let formApi: FormApi;
	export let field: FieldType;
	export let placeholder: string = '';
	export let required: boolean = false;
	export let helpText: string = '';
	export let id: string = '';
	export let rows: number = 3;
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
	<textarea
		{id}
		bind:value={$field.value}
		{placeholder}
		{rows}
		{disabled}
		class="{getInputClasses(showValidation && $field.errors.length > 0)} resize-vertical"
		on:blur={enableValidation}
		on:input={() => showValidation && field.validate()}
	></textarea>
</FormField>
