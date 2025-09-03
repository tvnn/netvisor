<script lang="ts">
  import { X, AlertCircle } from 'lucide-svelte';
  import { form as createForm } from 'svelte-forms';
  import { browser } from '$app/environment';
  
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

  $: if (browser && isOpen) {
    document.body.style.overflow = 'hidden';
  } else if (browser) {
    document.body.style.overflow = '';
  }
  
  // Also add cleanup in case the component is destroyed while modal is open
  import { onDestroy } from 'svelte';
  
  onDestroy(() => {
    if (browser) {
      document.body.style.overflow = '';
    }
  });
  
  // Size classes
  const sizeClasses = {
    sm: 'max-w-md',
    md: 'max-w-lg', 
    lg: 'max-w-2xl',
    xl: 'max-w-4xl',
    full: 'max-w-7xl'
  };
  
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
  
  function handleBackdropClick(event: MouseEvent) {
    if (!preventCloseOnClickOutside && event.target === event.currentTarget) {
      handleCancel();
    }
  }
  
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && isOpen) {
      handleCancel();
    }
  }
  
  // Disable save button if form validation fails or explicitly disabled
  $: actualDisableSave = disableSave || !$form.valid || loading || deleting;
  
  // Get all form errors for display
  $: formErrors = $form.errors || [];
</script>

<svelte:window on:keydown={handleKeydown} />

{#if isOpen}
  <!-- Modal backdrop -->
  <div 
    class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4 overscroll-contain"
    on:click={handleBackdropClick}
    role="dialog"
    aria-modal="true"
    aria-labelledby="modal-title"
    on:keydown={(e) => e.key === 'Escape' && handleCancel()}
    tabindex="-1"
  >
    <!-- Modal content -->
    <div class="bg-gray-900 rounded-lg shadow-2xl border border-gray-700 w-full {sizeClasses[size]} h-[95vh] flex flex-col">
      <form on:submit|preventDefault={handleFormSubmit} class="h-full flex flex-col">
        <!-- Header -->
        <div class="flex items-center justify-between p-6 border-b border-gray-700">
          <div class="flex items-center gap-3">
            <slot name="header-icon" />
            <h2 id="modal-title" class="text-xl font-semibold text-white">
              {title}
            </h2>
          </div>
          <button
            type="button"
            on:click={handleCancel}
            class="p-2 text-gray-400 hover:text-white hover:bg-gray-800 rounded-lg transition-colors"
            aria-label="Close modal"
          >
            <X class="w-5 h-5" />
          </button>
        </div>
        
        <!-- Content -->
        <div class="flex-1 overflow-hidden">
          <slot {form} />
        </div>
        
        <!-- Footer -->
        <div class="border-t border-gray-700 p-6 space-y-4">
          <!-- Validation errors display -->
          {#if formErrors.length > 0}
            <div class="rounded-lg bg-red-900/20 border border-red-600 p-4">
              <div class="flex items-start gap-3">
                <AlertCircle class="w-5 h-5 text-red-400 mt-0.5 flex-shrink-0" />
                <div>
                  <h5 class="text-sm font-medium text-red-400 mb-2">Please fix the following errors:</h5>
                  <ul class="text-sm text-red-300 space-y-1">
                    {#each formErrors as error}
                      <li class="flex items-start gap-2">
                        <span class="w-1 h-1 bg-red-400 rounded-full mt-2 flex-shrink-0"></span>
                        <span>{error}</span>
                      </li>
                    {/each}
                  </ul>
                </div>
              </div>
            </div>
          {/if}
          
          <!-- Action buttons -->
          <div class="flex justify-between">
            <!-- Delete button (left side) -->
            {#if onDelete}
              <button 
                type="button"
                on:click={handleDelete}
                disabled={loading || deleting}
                class="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 
                       disabled:bg-gray-600 disabled:cursor-not-allowed disabled:hover:bg-gray-600
                       transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 focus:ring-offset-gray-900"
              >
                {deleting ? 'Deleting...' : 'Delete'}
              </button>
            {:else}
              <div></div>
            {/if}
            
            <!-- Cancel and Save buttons (right side) -->
            <div class="flex gap-3">
              <button 
                type="button"
                on:click={handleCancel}
                disabled={loading || deleting}
                class="px-4 py-2 text-gray-400 hover:text-white hover:bg-gray-800 rounded-md transition-colors
                       disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {cancelLabel}
              </button>
              <button 
                type="submit"
                disabled={actualDisableSave}
                class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 
                       disabled:bg-gray-600 disabled:cursor-not-allowed disabled:hover:bg-gray-600
                       transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-gray-900"
              >
                {loading ? 'Saving...' : saveLabel}
              </button>
            </div>
          </div>
        </div>
      </form>
    </div>
  </div>
{/if}
    