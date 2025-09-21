import type { Writable } from 'svelte/store';

/**
 * Watch multiple stores and trigger a callback when any of them change
 * Automatically handles versioning to prevent triggering on initial subscription
 */
export function watchStores(
  stores: Writable<any>[],
  callback: () => void | Promise<void>
) {
  const lastVersions = new Map<Writable<any>, number>();
  
  // Initialize version tracking
  stores.forEach(store => {
    lastVersions.set(store, 0);
  });
  
  // Subscribe to each store
  const unsubscribes = stores.map(store => 
    store.subscribe(() => {
      const lastVersion = lastVersions.get(store) || 0;
      const currentVersion = Date.now();
      
      // Only trigger callback if this isn't the initial subscription
      if (lastVersion > 0) {
        callback();
      }
      
      lastVersions.set(store, currentVersion);
    })
  )

  return () => {
    unsubscribes.forEach(unsubscribe => unsubscribe());
  };
};