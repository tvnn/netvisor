import { get, writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import type { DiscoveryUpdatePayload, InitiateDiscoveryRequest } from './types/api';
import { pushError, pushSuccess, pushWarning } from '$lib/shared/stores/feedback';
import { getHosts } from '../hosts/store';
import { getSubnets } from '../subnets/store';
import { getServices } from '../services/store';
import { SSEClient, type SSEClient as SSEClientType } from '$lib/shared/utils/sse';

// session_id to latest update
export const sessions = writable<Map<string, DiscoveryUpdatePayload>>(new Map());
export const cancelling = writable<Map<string, boolean>>(new Map());

// Track last known discovered_count per session to detect changes
const lastDiscoveredCount = new Map<string, number>();

let sseClient: SSEClientType<DiscoveryUpdatePayload> | null = null;

export function startDiscoverySSE() {
	if (sseClient?.isConnected()) {
		return;
	}

	sseClient = new SSEClient<DiscoveryUpdatePayload>({
		url: '/api/discovery/stream',
		onMessage: (update) => {
			sessions.update((current) => {
				const newMap = new Map(current);
				newMap.set(update.session_id, update);

				// Check if discovered_count increased
				const lastCount = lastDiscoveredCount.get(update.session_id) || 0;
				const currentCount = update.discovered_count || 0;

				if (currentCount > lastCount) {
					// New hosts discovered - refresh data
					getHosts();
					getServices();
					getSubnets();
					lastDiscoveredCount.set(update.session_id, currentCount);
				}

				// Handle terminal phases
				if (update.phase === 'Complete') {
					pushSuccess(`Discovery completed with ${update.discovered_count} hosts found`);
					// Final refresh on completion
					getHosts();
					getServices();
					getSubnets();

					// Cleanup
					setTimeout(() => {
						sessions.update((s) => {
							const m = new Map(s);
							m.delete(update.session_id);
							lastDiscoveredCount.delete(update.session_id);
							return m;
						});
					}, 5000);
				} else if (update.phase === 'Cancelled') {
					pushWarning(`Discovery cancelled`);
					lastDiscoveredCount.delete(update.session_id);
					setTimeout(() => {
						sessions.update((s) => {
							const m = new Map(s);
							m.delete(update.session_id);
							return m;
						});
					}, 3000);
				} else if (update.phase === 'Failed' && update.error) {
					pushError(`Discovery error: ${update.error}`, -1);
					lastDiscoveredCount.delete(update.session_id);
				}

				// Clear cancelling state for terminal phases
				if (
					update.phase === 'Complete' ||
					update.phase === 'Cancelled' ||
					update.phase === 'Failed'
				) {
					cancelling.update((c) => {
						const m = new Map(c);
						m.delete(update.session_id);
						return m;
					});
				}

				return newMap;
			});
		},
		onError: (error) => {
			console.error('Discovery SSE error:', error);
			pushError('Lost connection to discovery updates');
		},
		onOpen: () => {
			console.log('Connected to discovery updates');
		}
	});

	sseClient.connect();
}

export function stopDiscoverySSE() {
	if (sseClient) {
		sseClient.disconnect();
		sseClient = null;
	}
}

export async function initiateDiscovery(data: InitiateDiscoveryRequest) {
	const result = await api.request<DiscoveryUpdatePayload, Map<string, DiscoveryUpdatePayload>>(
		'/discovery/initiate',
		sessions,
		(update, currentSessions) => {
			const map = new Map(currentSessions);
			map.set(update.session_id, update);
			return map;
		},
		{ method: 'POST', body: JSON.stringify(data) }
	);

	if (result?.success) {
		startDiscoverySSE(); // Start SSE on first discovery
	}
}

export async function cancelDiscovery(id: string) {
	const map = new Map(get(cancelling));
	map.set(id, true);
	cancelling.set(map);

	await api.request<void, void>(`/discovery/${id}/cancel`, null, null, { method: 'POST' });
}
