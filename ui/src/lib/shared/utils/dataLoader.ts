import { writable } from 'svelte/store';
import { pushError } from '../stores/feedback';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function loadData(loaders: (() => Promise<any>)[]) {
	const loading = writable(false);

	const loadingTimeout = setTimeout(() => {
		loading.set(true);
	}, 500);

	// Start loading immediately
	(async () => {
		try {
			await Promise.all(loaders.map((loader) => loader()));
			clearTimeout(loadingTimeout);
			loading.set(false);
		} catch (error) {
			pushError(`'Data loading failed:', ${error}`);
			clearTimeout(loadingTimeout);
			loading.set(false);
		}
	})();

	return loading;
}
