<script lang="ts">
	import { AlertCircle } from 'lucide-svelte';
	import { form as createForm } from 'svelte-forms';
	import GenericModal from '../layout/GenericModal.svelte';
	import type { FieldType, FormApi } from './types';

	export let title: string = 'Edit';
	export let isOpen: boolean = false;
	export let onSave: (() => void) | null = null;
	export let onCancel: (() => void) | null = null;
	export let saveLabel: string = 'Save';
	export let cancelLabel: string = 'Cancel';
	export let disableSave: boolean = false;
	export let size: 'sm' | 'md' | 'lg' | 'xl' | 'full' = 'lg';
	export let preventCloseOnClickOutside: boolean = false;
	export let loading: boolean = false;
	export let deleting: boolean = false;
	export let onDelete: (() => void) | null = null;
	export let isOnLastTab: boolean = true;

	// Create a container for fields that child components will populate
	let formFields: Record<string, FieldType> = {};

	// Create the actual form reactively based on registered fields
	$: form = createForm(...Object.values(formFields));
	let formErrors: string[] = [];

	const formApi: FormApi = {
		registerField: (id: string, field: FieldType) => {
			if (!formFields[id]) formFields = { ...formFields, [id]: field };
		},
		unregisterField: (id: string) => {
			if (formFields[id]) {
				let newFields = formFields;
				delete newFields[id];
				formFields = { ...newFields };
			}
		}
	};

	function handleFormSubmit() {
		// Only submit if form is valid
		if ($form.valid) {
			onSave?.();
		} else {
			formErrors = $form.errors;
		}
	}

	function handleCancel() {
		onCancel?.();
	}

	function handleDelete() {
		onDelete?.();
	}

	// Disable save button if form validation fails or explicitly disabled
	$: actualDisableSave = disableSave || loading || deleting || (isOnLastTab && !$form.valid);
</script>

<GenericModal {isOpen} {title} {size} {preventCloseOnClickOutside} onClose={handleCancel}>
	<!-- Header icon slot -->
	<svelte:fragment slot="header-icon">
		<slot name="header-icon" />
	</svelte:fragment>

	<!-- Main content -->
	<form on:submit|preventDefault={handleFormSubmit} class="flex h-full flex-col">
		<!-- Form content -->
		<div class="flex-1 overflow-auto p-6">
			<!-- Error display -->
			{#if formErrors.length > 0}
				<div class="mb-4 rounded-lg border border-red-700/30 bg-red-900/20 p-3">
					<div class="flex items-start gap-2">
						<AlertCircle class="mt-0.5 h-4 w-4 shrink-0 text-red-400" />
						<div>
							<p class="text-sm font-medium text-red-400">Please fix the following errors:</p>
							<ul class="mt-1 list-inside list-disc text-sm text-red-300">
								{#each formErrors as error (error)}
									<li>{error}</li>
								{/each}
							</ul>
						</div>
					</div>
				</div>
			{/if}

			<!-- Form fields slot -->
			<slot {form} {formApi} />
		</div>
	</form>

	<!-- Footer actions -->
	<svelte:fragment slot="footer">
		<div class="flex items-center justify-between">
			<!-- Delete button (if editing) -->
			<div>
				{#if onDelete}
					<button
						type="button"
						disabled={deleting || loading}
						on:click={handleDelete}
						class="rounded-lg border border-red-700/30 px-4 py-2
                   text-red-400 transition-colors hover:border-red-600/50 hover:bg-red-900/20
                   hover:text-red-300 disabled:cursor-not-allowed disabled:opacity-50"
					>
						{deleting ? 'Deleting...' : 'Delete'}
					</button>
				{/if}
			</div>

			<!-- Cancel and Save buttons -->
			<div class="flex items-center gap-3">
				{#if onCancel}
					<button
						type="button"
						disabled={loading || deleting}
						on:click={handleCancel}
						class="rounded-lg border border-gray-600 px-4 py-2
                   text-gray-400 transition-colors hover:bg-gray-700 hover:text-white
                   disabled:cursor-not-allowed disabled:opacity-50"
					>
						{cancelLabel}
					</button>
				{/if}

				<button
					type="button"
					disabled={actualDisableSave}
					on:click={onSave}
					class="rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors
                 hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
				>
					{loading ? 'Saving...' : saveLabel}
				</button>
			</div>
		</div>
	</svelte:fragment>
</GenericModal>
