import { get, writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import type { Network } from './types';
import { user } from '../users/store';

export const networks = writable<Network[]>([]);
export const currentNetwork = writable<Network>();

export async function getNetworks() {
	const currentUser = get(user);

	const result = await api.request<Network[]>(
		`/networks?user_id=${currentUser.id}`,
		networks,
		(networks) => networks,
		{ method: 'GET' }
	);

	if (result && result.success && result.data) {
		const current = get(networks).find((n) => n.is_default) || get(networks)[0];
		currentNetwork.set(current);
	}
}
