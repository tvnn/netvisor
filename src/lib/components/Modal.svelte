<script lang="ts">
  import { X } from 'lucide-svelte';
  import { modal, modalActions } from '../stores/ui';
  import { onMount } from 'svelte';

  let backdropElement: HTMLDivElement | undefined;

  onMount(() => {
    // Handle escape key and backdrop clicks
    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === 'Escape' && $modal.isOpen) {
        modalActions.close();
      }
    }

    function handleClick(event: MouseEvent): void {
      // Check if click is on the backdrop (not on modal content)
      if (backdropElement && event.target === backdropElement) {
        modalActions.close();
      }
    }

    if ($modal.isOpen) {
      document.addEventListener('keydown', handleKeydown);
      document.addEventListener('click', handleClick);
    }

    return () => {
      document.removeEventListener('keydown', handleKeydown);
      document.removeEventListener('click', handleClick);
    };
  });

  // Check if this is a small dialog (like ConfirmDialog)
  $: isSmallDialog = $modal.component?.name === 'ConfirmDialog' || $modal.title.includes('Confirm');

  // Re-setup event listeners when modal state changes
  $: if ($modal.isOpen) {
    // Event listeners are handled in onMount
  }
</script>

{#if $modal.isOpen}
  <!-- Modal backdrop -->
  <div 
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
    bind:this={backdropElement}
    role="dialog"
    aria-modal="true"
    aria-labelledby="modal-title"
    tabindex="-1"
  >
    {#if isSmallDialog}
      <!-- Small dialog - no wrapper, just the component -->
      <div role="document">
        {#if $modal.component}
          <svelte:component this={$modal.component} {...$modal.props} />
        {/if}
      </div>
    {:else}
      <!-- Regular modal with header -->
      <div 
        class="bg-gray-800 rounded-xl border border-gray-700 shadow-2xl max-w-4xl w-full max-h-[90vh] overflow-visible flex flex-col"
        role="document"
      >
        <!-- Modal header -->
        <div class="flex items-center justify-between p-6 border-b border-gray-700">
          <h2 id="modal-title" class="text-xl font-semibold text-white">
            {$modal.title}
          </h2>
          <button
            type="button"
            class="p-2 hover:bg-gray-700 rounded-lg transition-colors"
            on:click={modalActions.close}
            aria-label="Close modal"
          >
            <X class="w-5 h-5 text-gray-400" />
          </button>
        </div>
        
        <!-- Modal body - allow overflow for dropdowns -->
        <div class="flex-1 overflow-y-auto overflow-x-visible" style="overflow: visible;">
          {#if $modal.component}
            <svelte:component this={$modal.component} {...$modal.props} />
          {/if}
        </div>
      </div>
    {/if}
  </div>
{/if}