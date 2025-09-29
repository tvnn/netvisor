<script lang="ts">
  import { toastStore, dismissToast, type Toast } from "$lib/shared/stores/feedback";
  import { fly, fade } from 'svelte/transition';
  import { AlertCircle, AlertTriangle, Info, CheckCircle, X } from 'lucide-svelte';

  $: toasts = $toastStore;

  function getToastIcon(type: Toast['type']) {
    switch (type) {
      case 'error': return AlertCircle;
      case 'warning': return AlertTriangle;
      case 'info': return Info;
      case 'success': return CheckCircle;
      default: return Info;
    }
  }

  function getToastColors(type: Toast['type']) {
    switch (type) {
      case 'error':
        return {
          bg: 'bg-red-900/90',
          border: 'border-red-500/50',
          icon: 'text-red-400',
          text: 'text-red-100',
          button: 'text-red-300 hover:text-red-200'
        };
      case 'warning':
        return {
          bg: 'bg-yellow-900/90',
          border: 'border-yellow-500/50',
          icon: 'text-yellow-400',
          text: 'text-yellow-100',
          button: 'text-yellow-300 hover:text-yellow-200'
        };
      case 'info':
        return {
          bg: 'bg-blue-900/90',
          border: 'border-blue-500/50',
          icon: 'text-blue-400',
          text: 'text-blue-100',
          button: 'text-blue-300 hover:text-blue-200'
        };
      case 'success':
        return {
          bg: 'bg-green-900/90',
          border: 'border-green-500/50',
          icon: 'text-green-400',
          text: 'text-green-100',
          button: 'text-green-300 hover:text-green-200'
        };
      default:
        return {
          bg: 'bg-gray-900/90',
          border: 'border-gray-500/50',
          icon: 'text-gray-400',
          text: 'text-gray-100',
          button: 'text-gray-300 hover:text-gray-200'
        };
    }
  }

  function handleActionClick(action: () => void, toastId: string) {
    action();
    dismissToast(toastId);
  }
</script>

<!-- Toast Container -->
<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2 pointer-events-none">
  {#each toasts as toast (toast.id)}
    {@const colors = getToastColors(toast.type)}
    {@const IconComponent = getToastIcon(toast.type)}
    
    <div
      class="pointer-events-auto min-w-80 max-w-md rounded-lg border {colors.bg} {colors.border} shadow-lg backdrop-blur-sm"
      transition:fly="{{ x: 300, duration: 300 }}"
    >
      <div class="p-4">
        <div class="flex items-start gap-3">
          <!-- Icon -->
          <div class="flex-shrink-0 mt-0.5">
            <svelte:component this={IconComponent} class="w-5 h-5 {colors.icon}" />
          </div>
          
          <!-- Content -->
          <div class="flex-1 min-w-0">
            <p class="text-sm {colors.text} break-words">
              {toast.message}
            </p>
            
            <!-- Actions -->
            {#if toast.actions && toast.actions.length > 0}
              <div class="flex gap-2 mt-3">
                {#each toast.actions as action}
                  <button
                    type="button"
                    on:click={() => handleActionClick(action.action, toast.id)}
                    class="text-xs px-2 py-1 rounded {
                      action.style === 'primary'
                        ? 'bg-white/20 hover:bg-white/30 text-white'
                        : `${colors.button} hover:underline`
                    } transition-colors"
                  >
                    {action.label}
                  </button>
                {/each}
              </div>
            {/if}
          </div>
          
          <!-- Dismiss Button -->
          {#if toast.dismissible !== false}
            <button
              type="button"
              on:click={() => dismissToast(toast.id)}
              class="flex-shrink-0 p-1 rounded {colors.button} hover:bg-white/10 transition-colors"
              title="Dismiss"
            >
              <X class="w-4 h-4" />
            </button>
          {/if}
        </div>
      </div>
      
      <!-- Progress bar for timed toasts -->
      {#if toast.timeout && toast.timeout > 0}
        <div class="h-1 bg-white/20 rounded-b-lg overflow-hidden">
          <div 
            class="h-full bg-white/40 rounded-b-lg animate-shrink"
            style="animation-duration: {toast.timeout}ms"
          ></div>
        </div>
      {/if}
    </div>
  {/each}
</div>

<style>
  @keyframes shrink {
    from { width: 100%; }
    to { width: 0%; }
  }
  
  .animate-shrink {
    animation: shrink linear;
  }
</style>