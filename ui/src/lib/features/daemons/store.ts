import { derived, get, writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import type { Daemon } from './types/base';
import type { DaemonDiscoveryUpdate } from '../discovery/types/api';
import { hosts } from '../hosts/store';
import type { Host } from '../hosts/types/base';

export const daemons = writable<Daemon[]>([]);

export async function getDaemons() {
	return await api.request<Daemon[]>('/daemons', daemons, (daemons) => daemons, { method: 'GET' });
}

export function getDaemonDiscoveryState(
	daemon_id: string | null,
	sessionsMap: Map<string, DaemonDiscoveryUpdate>
): DaemonDiscoveryUpdate | null {
	if (!daemon_id) return null;
	return sessionsMap.get(daemon_id) || null;
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
