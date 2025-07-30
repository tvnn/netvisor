<script lang="ts">
  import { AlertTriangle, X } from 'lucide-svelte';
  import { modalActions } from '../../stores/ui';

  export let title = 'Confirm Action';
  export let message = 'Are you sure you want to proceed?';
  export let confirmText = 'Confirm';
  export let cancelText = 'Cancel';
  export let onConfirm: () => void | Promise<void> = () => {};
  export let danger = false;

  async function handleConfirm() {
    try {
      await onConfirm();
      modalActions.close();
    } catch (error) {
      // Don't close modal if there's an error
      console.error('Confirmation action failed:', error);
    }
  }

  function handleCancel() {
    modalActions.close();
  }
</script>

<div class="bg-gray-800 rounded-xl border border-gray-700 w-96 max-w-[90vw] mx-auto shadow-2xl">
  <!-- Header -->
  <div class="flex items-center justify-between p-5 border-b border-gray-700">
    <h3 class="text-lg font-semibold text-white">{title}</h3>
    <button
      on:click={handleCancel}
      class="p-1 hover:bg-gray-700 rounded-lg text-gray-400 hover:text-white transition-colors"
    >
      <X class="w-4 h-4" />
    </button>
  </div>

  <!-- Content -->
  <div class="p-5">
    <!-- Icon and Message -->
    <div class="flex items-start gap-4 mb-6">
      <div class="p-2 {danger ? 'bg-red-600/20 border border-red-600/30' : 'bg-yellow-600/20 border border-yellow-600/30'} rounded-lg shrink-0">
        <AlertTriangle class="w-5 h-5 {danger ? 'text-red-400' : 'text-yellow-400'}" />
      </div>
      <p class="text-gray-300 leading-relaxed">
        {message}
      </p>
    </div>

    <!-- Actions -->
    <div class="flex gap-3 justify-end">
      <button
        type="button"
        on:click={handleCancel}
        class="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition-colors"
      >
        {cancelText}
      </button>
      <button
        type="button"
        on:click={handleConfirm}
        class="px-4 py-2 {danger ? 'bg-red-600 hover:bg-red-700' : 'bg-blue-600 hover:bg-blue-700'} text-white rounded-lg transition-colors"
      >
        {confirmText}
      </button>
    </div>
  </div>
</div>