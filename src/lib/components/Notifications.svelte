<script lang="ts">
  import { X, CheckCircle, XCircle, AlertTriangle, Info } from 'lucide-svelte';
  import { notifications, notificationActions } from '../stores/ui';
  import type { NotificationItem } from '../types';

  function getIcon(type: NotificationItem['type']) {
    switch (type) {
      case 'success': return CheckCircle;
      case 'error': return XCircle;
      case 'warning': return AlertTriangle;
      case 'info': return Info;
      default: return Info;
    }
  }

  function getColorClasses(type: NotificationItem['type']) {
    switch (type) {
      case 'success': return 'bg-green-600 border-green-500 text-white';
      case 'error': return 'bg-red-600 border-red-500 text-white';
      case 'warning': return 'bg-yellow-600 border-yellow-500 text-white';
      case 'info': return 'bg-blue-600 border-blue-500 text-white';
      default: return 'bg-gray-600 border-gray-500 text-white';
    }
  }

  function dismiss(id: string) {
    notificationActions.remove(id);
  }
</script>

<!-- Notification Container -->
<div class="fixed bottom-4 right-4 z-50 space-y-2 max-w-sm">
  {#each $notifications as notification (notification.id)}
    <div 
      class="flex items-start gap-3 p-4 rounded-lg border shadow-lg transition-all duration-300 ease-in-out {getColorClasses(notification.type)}"
      style="animation: slideInRight 0.3s ease-out"
    >
      <!-- Icon -->
      <svelte:component this={getIcon(notification.type)} class="w-5 h-5 mt-0.5 flex-shrink-0" />
      
      <!-- Message -->
      <div class="flex-1 min-w-0">
        <p class="text-sm font-medium leading-tight">
          {notification.message}
        </p>
      </div>
      
      <!-- Dismiss button -->
      <button
        on:click={() => dismiss(notification.id)}
        class="p-1 hover:bg-white/20 rounded transition-colors flex-shrink-0"
        aria-label="Dismiss notification"
      >
        <X class="w-4 h-4" />
      </button>
    </div>
  {/each}
</div>

<style>
  @keyframes slideInRight {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }
</style>