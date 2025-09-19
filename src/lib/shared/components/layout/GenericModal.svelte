<script lang="ts">
  import { X } from 'lucide-svelte';
  import { onDestroy } from 'svelte';
  
  export let title: string = 'Modal';
  export let isOpen: boolean = false;
  export let onClose: (() => void) | null = null;
  export let size: 'sm' | 'md' | 'lg' | 'xl' | 'full' = 'lg';
  export let preventCloseOnClickOutside: boolean = false;
  export let showCloseButton: boolean = true;

  $: if (typeof window !== "undefined" && isOpen) {
    document.body.style.overflow = 'hidden';
  } else if (typeof window !== "undefined") {
    document.body.style.overflow = '';
  }
  
  onDestroy(() => {
    if (typeof window !== "undefined") {
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
  
  function handleClose() {
    onClose?.();
  }
  
  function handleBackdropClick(event: MouseEvent) {
    if (!preventCloseOnClickOutside && event.target === event.currentTarget) {
      handleClose();
    }
  }
  
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && isOpen) {
      handleClose();
    }
  }
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
    on:keydown={(e) => e.key === 'Escape' && handleClose()}
    tabindex="-1"
  >
    <!-- Modal content -->
    <div class="bg-gray-900 rounded-lg shadow-2xl border border-gray-700 w-full {sizeClasses[size]} max-h-[95vh] flex flex-col">
      <!-- Header -->
      <div class="flex items-center justify-between p-6 border-b border-gray-700 shrink-0">
        <div class="flex items-center gap-3">
          {#if $$slots['header-icon']}
            <slot name="header-icon" />
          {/if}
          <h2 id="modal-title" class="text-xl font-semibold text-white">
            {title}
          </h2>
        </div>
        
        {#if showCloseButton}
          <button
            type="button"
            on:click={handleClose}
            class="text-gray-400 hover:text-white p-2 hover:bg-gray-700 rounded-lg transition-colors"
            aria-label="Close modal"
          >
            <X class="w-5 h-5" />
          </button>
        {/if}
      </div>

      <!-- Content slot -->
      <div class="flex-1 overflow-auto min-h-0">
        <slot />
      </div>

      <!-- Footer slot -->
      {#if $$slots.footer}
        <div class="border-t border-gray-700 p-6 shrink-0">
          <slot name="footer" />
        </div>
      {/if}
    </div>
  </div>
{/if}