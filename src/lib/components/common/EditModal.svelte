<script lang="ts">
  import { X, Trash2 } from 'lucide-svelte';
  
  export let isOpen = false;
  export let title = '';
  export let loading = false;
  export let deleting = false;
  export let submitLabel = 'Save';
  export let cancelLabel = 'Cancel';
  export let onSubmit: (data: any) => Promise<void> | void;
  export let onClose: () => void;
  export let onCancel: () => void;
  export let onDelete: (() => Promise<void> | void) | null = null;
  
  async function handleSubmit(event: Event) {
    event.preventDefault();
    const formData = new FormData(event.target as HTMLFormElement);
    const data = Object.fromEntries(formData.entries());
    
    await onSubmit(data);
  }
  
  function handleClose() {
    if (!loading && !deleting) {
      onClose();
    }
  }
  
  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      handleClose();
    }
  }
</script>

{#if isOpen}
  <div 
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    on:click={handleBackdropClick}
    on:keydown={(e) => e.key === 'Escape' && handleClose()}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <!-- Changed: Use flexbox layout with fixed height and no overflow on main container -->
    <div class="bg-gray-800 rounded-lg shadow-xl mx-4 max-w-6xl w-[90vw] h-[90vh] flex flex-col">
      
      <!-- Fixed Header -->
      <div class="flex items-center justify-between p-6 border-b border-gray-700 flex-shrink-0">
        <h2 class="text-xl font-semibold text-white">{title}</h2>
        <button
          on:click={handleClose}
          disabled={loading || deleting}
          class="text-gray-400 hover:text-white transition-colors disabled:opacity-50"
        >
          <X size={24} />
        </button>
      </div>
      
      <!-- Scrollable Content Area -->
      <form on:submit={handleSubmit} class="flex-1 flex flex-col min-h-0">
        <div class="flex-1 overflow-y-auto p-6 min-h-0">
          <slot />
        </div>
        
        <!-- Fixed Footer - now outside the scrollable area -->
        <div class="flex-shrink-0 border-t border-gray-700 p-6">
          <div class="flex justify-between items-center">
            <!-- Delete button (left side) -->
            <div>
              {#if onDelete}
                <button
                  type="button"
                  on:click={onDelete}
                  disabled={deleting || loading}
                  class="flex items-center gap-2 px-4 py-2 text-red-300 hover:text-red-200 border border-red-600 rounded-md hover:border-red-500 transition-colors disabled:opacity-50"
                >
                  <Trash2 size={16} />
                  {deleting ? 'Deleting...' : 'Delete'}
                </button>
              {/if}
            </div>
            
            <!-- Save/Cancel buttons (right side) -->
            <div class="flex gap-3">
              <button
                type="button"
                on:click={onCancel}
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
        </div>
      </form>
    </div>
  </div>
{/if}