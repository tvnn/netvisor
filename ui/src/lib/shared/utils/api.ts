import type { Writable } from 'svelte/store';
import { pushError } from '../stores/feedback';
import { PUBLIC_SERVER_HOSTNAME, PUBLIC_SERVER_PORT } from '$env/static/public';

interface ApiResponse<T> {
	success: boolean;
	data?: T;
	error?: string;
}

interface RequestCache {
	promise: Promise<any>;
	timestamp: number;
	completed: boolean;
	result?: any;
}

class ApiClient {
	private requestCache = new Map<string, RequestCache>();
	private debounceMs: number = 250;

	private getRequestKey(endpoint: string, method: string, body?: string): string {
		// For GET requests, just use endpoint + method
		// For POST/PUT/DELETE, include body hash to allow different payloads
		if (method === 'GET') {
			return `${method}:${endpoint}`;
		}
		return `${method}:${endpoint}:${body || ''}`;
	}

	private cleanupExpiredRequests() {
		const now = Date.now();
		for (const [key, cache] of this.requestCache.entries()) {
			if (now - cache.timestamp > this.debounceMs) {
				this.requestCache.delete(key);
			}
		}
	}

	async request<TResponseData, TStoreData = TResponseData>(
		endpoint: string,
		dataStore: Writable<TStoreData> | null,
		storeAction: ((data: TResponseData, current: TStoreData) => TStoreData) | null,
		options: RequestInit = {},
		isBackgroundRequest: boolean = false
	): Promise<ApiResponse<TResponseData> | null> {
		const method = options.method || 'GET';
		const body = options.body as string;
		const requestKey = this.getRequestKey(endpoint, method, body);

		// Clean up expired requests first
		this.cleanupExpiredRequests();

		// Check if we have a cached request within the debounce window
		const cached = this.requestCache.get(requestKey);
		if (cached) {
			const timeSinceRequest = Date.now() - cached.timestamp;
			if (timeSinceRequest < this.debounceMs) {
				if (cached.completed && cached.result) {
					// Return cached result immediately if request completed
					if (dataStore && storeAction && cached.result.success) {
						dataStore.update((current) => storeAction(cached.result.data!, current));
					}
					return cached.result;
				} else {
					// Return the pending promise
					return cached.promise;
				}
			}
		}

		let hostname =
			PUBLIC_SERVER_HOSTNAME == 'default' ? window.location.hostname : PUBLIC_SERVER_HOSTNAME;

		const url = URL.parse(
			`${window.location.protocol}/${hostname}:${PUBLIC_SERVER_PORT}/api${endpoint}`
		);
		const baseErrorMessage = `Failed to ${method} from ${endpoint}`;

		if (!url) {
			pushError('Invalid url');
			return null;
		}

		const requestPromise = this.executeRequest<TResponseData, TStoreData>(
			url,
			dataStore,
			storeAction,
			options,
			baseErrorMessage
		);

		// Cache the request
		const cacheEntry: RequestCache = {
			promise: requestPromise,
			timestamp: Date.now(),
			completed: false,
			result: undefined
		};

		this.requestCache.set(requestKey, cacheEntry);

		// Store the result when the request completes
		requestPromise
			.then((result) => {
				cacheEntry.completed = true;
				cacheEntry.result = result;
			})
			.catch((error) => {
				cacheEntry.completed = true;
				cacheEntry.result = null;
			});

		return requestPromise;
	}

	private async executeRequest<TResponseData, TStoreData = TResponseData>(
		url: URL,
		dataStore: Writable<TStoreData> | null,
		storeAction: ((data: TResponseData, current: TStoreData) => TStoreData) | null,
		options: RequestInit,
		baseErrorMessage: string
	): Promise<ApiResponse<TResponseData> | null> {
		try {
			const response = await fetch(url, {
				headers: {
					'Content-Type': 'application/json',
					...options.headers
				},
				...options
			});

			if (!response.ok) {
				const errorData = await response.json().catch(() => ({
					success: false,
					error: `HTTP ${response.status}: ${response.statusText}`
				}));
				const errorMsg = errorData.error || `HTTP ${response.status}`;
				pushError(errorMsg);
				return null;
			}

			const jsonResponse: ApiResponse<TResponseData> = await response.json();
			if (jsonResponse.success) {
				if (dataStore && storeAction) {
					dataStore.update((current) => storeAction(jsonResponse.data!, current));
				}
				return jsonResponse;
			} else if (jsonResponse?.error) {
				pushError(`${baseErrorMessage}: ${jsonResponse.error}`);
				return null;
			} else {
				pushError(`${baseErrorMessage}: Unknown error`);
				return null;
			}
		} catch (err) {
			pushError(`${baseErrorMessage}: ${err}`);
			return null;
		}
	}

	// Allow configuration of debounce interval
	setDebounceInterval(ms: number) {
		this.debounceMs = ms;
	}

	// Method to clear cache manually if needed
	clearCache() {
		this.requestCache.clear();
	}
}

export const api = new ApiClient();
