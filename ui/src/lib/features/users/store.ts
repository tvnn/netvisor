import { writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import type { User } from './types';
import { pushError, pushWarning } from '$lib/shared/stores/feedback';
import { utcTimeZoneSentinel, uuidv4Sentinel } from '$lib/shared/utils/formatting';

export const user = writable<User>();

export async function getUser(user_id: string): Promise<User | null> {
	const result = await api.request<User>(`/users/${user_id}`, user, (user) => user, {
		method: 'GET'
	});

	if (result && result.success && result.data) {
		return result.data;
	}
	return null;
}

export async function createUser(): Promise<User | null> {
	const result = await api.request<User>('/users', user, (user) => user, {
		method: 'POST',
		body: JSON.stringify(newUser())
	});

	if (result && result.success && result.data) {
		return result.data;
	}
	return null;
}

export async function loadUser(retry_count: number = 0): Promise<User | null> {
	const user_id = localStorage.getItem('user_id');

	// No stored user - create new one
	if (user_id == null) {
		const user = await createUser();
		if (user) {
			localStorage.setItem('user_id', user.id);
			return user;
		}

		// Only retry creation, don't recurse infinitely
		if (retry_count < 3) {
			pushWarning(`Failed to create user. Retrying... (${retry_count + 1}/3)`);
			return loadUser(retry_count + 1);
		}

		pushError('Failed to create user after 3 attempts.');
		return null;
	}

	// Have stored user - try to fetch from server
	const user = await getUser(user_id);
	if (user) {
		return user;
	}

	// User not found on server
	if (retry_count < 3) {
		pushWarning(`Could not find user ${user_id}. Retrying... (${retry_count + 1}/3)`);
		return loadUser(retry_count + 1);
	}

	// After retries failed, decide what to do
	pushError(`Could not load user ${user_id} after 3 attempts. This may be a network issue.`);

	return null;
}

function newUser() {
	const user: User = {
		id: uuidv4Sentinel,
		created_at: utcTimeZoneSentinel,
		updated_at: utcTimeZoneSentinel,
		name: ''
	};

	return user;
}
