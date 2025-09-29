import { writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import { createPoller, Poller } from '../../shared/utils/polling';
import type { DaemonDiscoveryUpdate, InitiateDiscoveryRequest } from './types/api';
import { pushError, pushSuccess, pushWarning } from '$lib/shared/stores/feedback';
import { getHosts } from '../hosts/store';
import { getSubnets } from '../subnets/store';
import { getServices } from '../services/store';

// daemon_id to latest update
export const sessions = writable<Map<string, DaemonDiscoveryUpdate>>(new Map());

// Discovery status poller instance
let discoveryPoller: Poller | null = null;

export function startDiscoveryPolling() {
	discoveryPoller = createPoller({
		intervalMs: 5000, // 5 seconds
		onPoll: async () => {
			Promise.all([
				await getActiveDiscoverySessions(),
				await getServices(),
				await getSubnets(),
				await getHosts()
			]);
		},
		onError: (pollingError) => {
			pushError(`Failed to poll discovery status: ${pollingError}`);
			stopDiscoveryPolling();
		},
		name: 'DiscoveryPoller'
	});

	discoveryPoller.start();
}

export async function stopDiscoveryPolling() {
	if (discoveryPoller) {
		discoveryPoller.stop();
		discoveryPoller = null;
	}
}

export async function initiateDiscovery(data: InitiateDiscoveryRequest) {
	const result = await api.request<DaemonDiscoveryUpdate, Map<string, DaemonDiscoveryUpdate>>(
		'/discovery/initiate',
		sessions,
		(update, currentSessions) => {
			const map = new Map(currentSessions);
			map.set(update.daemon_id, update);
			return map;
		},
		{ method: 'POST', body: JSON.stringify(data) },
		true
	);

	if (result?.success && !discoveryPoller?.getIsRunning) {
		startDiscoveryPolling();
	}
}

export async function cancelDiscovery(id: string) {
	await api.request<void, Map<string, DaemonDiscoveryUpdate>>(
		`/discovery/${id}/cancel`,
		null,
		null,
		{ method: 'POST' },
		true
	);
}

export async function getActiveDiscoverySessions() {
	const result = await api.request<DaemonDiscoveryUpdate[], Map<string, DaemonDiscoveryUpdate>>(
		'/discovery/active',
		sessions,
		(actives, current) => {
			// Only update if there are actual changes
			const newMap = actives.reduce((map, session) => {
				map.set(session.daemon_id, session);
				return map;
			}, new Map<string, DaemonDiscoveryUpdate>());

			// Compare with current state - only return new map if different
			if (current.size !== newMap.size) {
				return newMap;
			}

			// Check for actual content changes
			for (const [daemonId, session] of newMap) {
				const currentSession = current.get(daemonId);
				if (
					!currentSession ||
					currentSession.completed !== session.completed ||
					currentSession.phase !== session.phase ||
					currentSession.discovered_count !== session.discovered_count ||
					currentSession.error !== session.error
				) {
					if (session.phase == 'Complete')
						pushSuccess(`"Discovery completed with ${session.discovered_count} hosts found`);
					if (session.phase == 'Warn')
						pushWarning(`"Discovery cancelled with ${session.discovered_count} hosts found`);
					if (session.error) pushError(`"Discovery error: ${session.error}`);

					return newMap;
				}
			}

			// No changes - return current to prevent reactive updates
			return current;
		},
		{ method: 'GET' },
		true
	);

	if (result?.success && result.data && result.data?.length > 0) {
		if (!discoveryPoller?.getIsRunning()) startDiscoveryPolling();
	} else {
		stopDiscoveryPolling();
	}
}
