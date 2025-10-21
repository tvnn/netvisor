import { derived, get, writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import type { Daemon } from './types/base';
import type { DiscoveryUpdatePayload } from '../discovery/types/api';
import { hosts } from '../hosts/store';
import type { Host } from '../hosts/types/base';
import { currentNetwork } from '../networks/store';

export const daemons = writable<Daemon[]>([]);

export async function getDaemons() {
	return await api.request<Daemon[]>(
		`/daemons?network_id=${get(currentNetwork).id}`,
		daemons,
		(daemons) => daemons,
		{ method: 'GET' }
	);
}

export function getDaemonIsRunningDiscovery(
	daemon_id: string | null,
	sessionsMap: Map<string, DiscoveryUpdatePayload>
): boolean {
	if (!daemon_id) return false;

	// Find any active session for this daemon
	for (const session of sessionsMap.values()) {
		if (
			session.daemon_id === daemon_id &&
			(session.phase === 'Initiated' || session.phase === 'Started' || session.phase === 'Scanning')
		) {
			return true;
		}
	}
	return false;
}

export function getDaemonDiscoveryData(
	daemonId: string,
	sessions: Map<string, DiscoveryUpdatePayload>
): DiscoveryUpdatePayload | null {
	// Find the active session for this daemon
	for (const session of sessions.values()) {
		if (
			session.daemon_id === daemonId &&
			(session.phase === 'Initiated' || session.phase === 'Started' || session.phase === 'Scanning')
		) {
			return session;
		}
	}
	return null;
}

export function getDaemonHost(daemon_id: string): Host | null {
	const daemon = get(daemons).find((d) => d.id === daemon_id);
	if (!daemon) return null;

	const host = get(hosts).find((n) => n.id === daemon.host_id) || null;
	return host ? host : null;
}

export const hostDaemonMap = derived(daemons, ($daemons) => {
	const map = new Map<string, Daemon>();
	$daemons.forEach((daemon) => {
		map.set(daemon.host_id, daemon);
	});
	return map;
});
