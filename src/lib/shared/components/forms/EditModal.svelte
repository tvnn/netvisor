<script lang="ts">
  import { AlertCircle } from 'lucide-svelte';
  import { form as createForm } from 'svelte-forms';
	import GenericModal from '../layout/GenericModal.svelte';
  
  export let title: string = 'Edit';
  export let isOpen: boolean = false;
  export let onSave: (() => void) | null = null;
  export let onCancel: (() => void) | null = null;
  export let saveLabel: string = "Save";
  export let cancelLabel: string = "Cancel";
  export let disableSave: boolean = false;
  export let size: 'sm' | 'md' | 'lg' | 'xl' | 'full' = 'lg';
  export let preventCloseOnClickOutside: boolean = false;
  export let loading: boolean = false;
  export let deleting: boolean = false;
  export let onDelete: (() => void) | null = null;
  
  // Create form instance
  const form = createForm();
  
  function handleFormSubmit() {
    // Only submit if form is valid
    if ($form.valid) {
      onSave?.();
    }
  }
  
  function handleCancel() {
    onCancel?.();
  }
  
  function handleDelete() {
    onDelete?.();
  }
  
  // Disable save button if form validation fails or explicitly disabled
  $: actualDisableSave = disableSave || !$form.valid || loading || deleting;
  
  // Get all form errors for display
  $: formErrors = $form.errors || [];
</script>

<GenericModal 
  {isOpen} 
  {title} 
  {size} 
  {preventCloseOnClickOutside}
  onClose={handleCancel}
>
  <!-- Header icon slot -->
  <svelte:fragment slot="header-icon">
    <slot name="header-icon" />
  </svelte:fragment>

  <!-- Main content -->
  <form on:submit|preventDefault={handleFormSubmit} class="h-full flex flex-col">
    <!-- Form content -->
    <div class="flex-1 overflow-auto p-6">
      <!-- Error display -->
      {#if formErrors.length > 0}
        <div class="mb-4 p-3 bg-red-900/20 border border-red-700/30 rounded-lg">
          <div class="flex items-start gap-2">
            <AlertCircle class="w-4 h-4 text-red-400 mt-0.5 shrink-0" />
            <div>
              <p class="text-red-400 text-sm font-medium">Please fix the following errors:</p>
              <ul class="mt-1 text-red-300 text-sm list-disc list-inside">
                {#each formErrors as error}
                  <li>{error}</li>
                {/each}
              </ul>
            </div>
          </div>
        </div>
      {/if}

      <!-- Form fields slot -->
      <slot {form} />
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
            class="px-4 py-2 text-red-400 hover:text-red-300 hover:bg-red-900/20 
                   border border-red-700/30 hover:border-red-600/50 rounded-lg 
                   transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
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
            class="px-4 py-2 text-gray-400 hover:text-white hover:bg-gray-700 
                   border border-gray-600 rounded-lg transition-colors
                   disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {cancelLabel}
          </button>
        {/if}
        
        <button
          type="button"
          disabled={actualDisableSave}
          on:click={onSave}
          class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg 
                 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {loading ? 'Saving...' : saveLabel}
        </button>
      </div>
    </div>
  </svelte:fragment>
</GenericModal>