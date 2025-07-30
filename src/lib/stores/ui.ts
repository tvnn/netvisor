import { writable, derived, type Writable, type Readable } from 'svelte/store';
import type { NotificationItem, ModalState, LoadingState } from '../types';

// UI state management
export const activeTab: Writable<string> = writable('diagnostics');

// Modal state
export const modal: Writable<ModalState> = writable({
  isOpen: false,
  component: null,
  props: {},
  title: ''
});

// Notification system
export const notifications: Writable<NotificationItem[]> = writable([]);

// App-wide loading states
export const loading: Writable<LoadingState> = writable({
  global: false,
  diagnostics: false,
  nodes: false,
  tests: false
});

// Modal management functions
export const modalActions = {
  open(component: any, props: Record<string, any> = {}, title: string = ''): void {
    modal.set({
      isOpen: true,
      component,
      props,
      title
    });
  },
  
  close(): void {
    modal.set({
      isOpen: false,
      component: null,
      props: {},
      title: ''
    });
  },
  
  updateProps(newProps: Record<string, any>): void {
    modal.update(current => ({
      ...current,
      props: { ...current.props, ...newProps }
    }));
  }
};

// Notification management
export const notificationActions = {
  add(message: string, type: NotificationItem['type'] = 'info', duration: number = 5000): string {
    const id = crypto.randomUUID();
    const notification: NotificationItem = {
      id,
      message,
      type,
      duration,
      createdAt: Date.now()
    };
    
    notifications.update(current => [...current, notification]);
    
    // Auto-remove after duration
    if (duration > 0) {
      setTimeout(() => {
        this.remove(id);
      }, duration);
    }
    
    return id;
  },
  
  remove(id: string): void {
    notifications.update(current => current.filter(n => n.id !== id));
  },
  
  clear(): void {
    notifications.set([]);
  },
  
  success(message: string, duration: number = 5000): string {
    return this.add(message, 'success', duration);
  },
  
  error(message: string, duration: number = 8000): string {
    return this.add(message, 'error', duration);
  },
  
  warning(message: string, duration: number = 6000): string {
    return this.add(message, 'warning', duration);
  },
  
  info(message: string, duration: number = 5000): string {
    return this.add(message, 'info', duration);
  }
};

// Loading state management
export const loadingActions = {
  setGlobal(isLoading: boolean): void {
    loading.update(current => ({ ...current, global: isLoading }));
  },
  
  setDiagnostics(isLoading: boolean): void {
    loading.update(current => ({ ...current, diagnostics: isLoading }));
  },
  
  setNodes(isLoading: boolean): void {
    loading.update(current => ({ ...current, nodes: isLoading }));
  },
  
  setTests(isLoading: boolean): void {
    loading.update(current => ({ ...current, tests: isLoading }));
  }
};

// Keyboard shortcuts
export const shortcuts: Writable<Record<string, () => void>> = writable({
  'ctrl+1': () => activeTab.set('diagnostics'),
  'ctrl+2': () => activeTab.set('nodes'),
  'ctrl+3': () => activeTab.set('tests'),
  'escape': () => modalActions.close()
});

// Derived stores
export const isAnyLoading: Readable<boolean> = derived(loading, $loading => 
  Object.values($loading).some(Boolean)
);

export const hasNotifications: Readable<boolean> = derived(notifications, $notifications => 
  $notifications.length > 0
);