<!-- src/lib/components/common/GenericEditModal.svelte -->
<script lang="ts">
  import { X } from 'lucide-svelte';
  
  export let isOpen = false;
  export let title: string;
  export let loading = false;
  export let onSubmit: (data: any) => Promise<void> | void;
  export let onClose: () => void;
  export let submitLabel: string = 'Save';
  export let cancelLabel: string = 'Cancel';
  export let deleteLabel: string = 'Delete';
  export let onDelete: (() => Promise<void> | void) | null = null;
  export let deleting = false;
  
  let formElement: HTMLFormElement;
  
  async function handleSubmit(event: Event) {
    event.preventDefault();
    const formData = new FormData(formElement);
    const data = Object.fromEntries(formData.entries());
    await onSubmit(data);
  }
  
  async function handleDelete() {
    if (onDelete) {
      await onDelete();
    }
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-gray-800 rounded-lg p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto">
      <div class="flex justify-between items-center mb-6">
        <h2 class="text-xl font-semibold text-white">{title}</h2>
        <button
          on:click={onClose}
          class="text-gray-400 hover:text-white"
        >
          <X size={24} />
        </button>
      </div>
      
      <form bind:this={formElement} on:submit={handleSubmit} class="space-y-6">
        <!-- Content slot -->
        <slot />
        
        <!-- Action Buttons -->
        <div class="flex justify-between pt-4">
          <!-- Delete button (if provided) -->
          <div>
            {#if onDelete}
              <button
                type="button"
                on:click={handleDelete}
                disabled={deleting || loading}
                class="flex items-center gap-2 px-4 py-2 text-red-300 hover:text-red-200 border border-red-600 rounded-md hover:border-red-500 transition-colors disabled:opacity-50"
              >
                {deleting ? 'Deleting...' : deleteLabel}
              </button>
            {/if}
          </div>
          
          <!-- Save/Cancel buttons -->
          <div class="flex space-x-3">
            <button
              type="button"
              on:click={onClose}
              disabled={loading || deleting}
              class="px-4 py-2 text-gray-300 hover:text-white border border-gray-600 rounded-md hover:border-gray-500 transition-colors disabled:opacity-50"
            >
              {cancelLabel}
            </button>
            <button
              type="submit"
              disabled={loading || deleting}
              class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed transition-colors"
            >
              {loading ? 'Saving...' : submitLabel}
            </button>
          </div>
        </div>
      </form>
    </div>
  </div>
{/if}