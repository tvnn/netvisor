<script context="module" lang="ts">
	export const INPUT_BASE_CLASSES =
		'w-full px-3 py-2 bg-gray-700 border rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed';
	export const getInputClasses = (hasErrors: boolean) =>
		`${INPUT_BASE_CLASSES} ${hasErrors ? 'border-red-500' : 'border-gray-600'}`;
</script>

<!-- FormField.svelte - Base form field wrapper -->
<script lang="ts">
	import { AlertCircle } from 'lucide-svelte';
	import type { FieldType, FormApi } from '../types';
	import { onMount } from 'svelte';

	export let label: string;
	export let formApi: FormApi;
	export let field: FieldType;
	export let required: boolean = false;
	export let helpText: string = '';
	export let errors: string[] = [];
	export let showValidation: boolean = true;
	export let id: string = '';

	onMount(() => {
		formApi.registerField(id, field);
	});
</script>

<div class="space-y-2">
	<label for={id} class="block text-sm font-medium text-gray-300">
		{label}
		{#if required}
			<span class="ml-1 text-red-400">*</span>
		{/if}
	</label>

	<slot />

	{#if showValidation && errors.length > 0}
		<div class="flex items-center gap-2 text-red-400">
			<AlertCircle size={16} />
			<p class="text-xs">{errors[0]}</p>
		</div>
	{/if}

	{#if helpText}
		<p class="text-xs text-gray-400">{helpText}</p>
	{/if}
</div>
