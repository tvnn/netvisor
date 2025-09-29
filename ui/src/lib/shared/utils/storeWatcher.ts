import type { Writable } from 'svelte/store';

/**
 * Watch multiple stores and trigger a callback when any of them change
 * Automatically handles versioning to prevent triggering on initial subscription
 * Debounces callbacks by default to prevent cascading updates
 */
export function watchStores(
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	stores: Writable<any>[],
	callback: () => void | Promise<void>,
	debounceMs: number = 100 // Default 100ms debounce
) {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const lastVersions = new Map<Writable<any>, number>();
	let timeoutId: ReturnType<typeof setTimeout> | null = null;

	// Initialize version tracking
	stores.forEach((store) => {
		lastVersions.set(store, 0);
	});

	const debouncedCallback = () => {
		if (timeoutId) {
			clearTimeout(timeoutId);
		}

		timeoutId = setTimeout(() => {
			callback();
			timeoutId = null;
		}, debounceMs);
	};

	// Subscribe to each store
	const unsubscribes = stores.map((store) =>
		store.subscribe(() => {
			const lastVersion = lastVersions.get(store) || 0;
			const currentVersion = Date.now();

			// Only trigger callback if this isn't the initial subscription
			if (lastVersion > 0) {
				debouncedCallback();
			}

			lastVersions.set(store, currentVersion);
		})
	);

	// Return cleanup function that also clears any pending timeout
	return () => {
		if (timeoutId) {
			clearTimeout(timeoutId);
		}
		unsubscribes.forEach((unsubscribe) => unsubscribe());
	};
}
