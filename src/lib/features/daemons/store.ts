import { writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import type { Daemon } from "./types/base";

export const daemons = writable<Daemon[]>([]);
export const loading = writable(false);
export const error = writable<string | null>(null);

export async function getDaemons() {
  return await api.request<Daemon[]>(
    '/daemons',
    daemons,
    (daemons) => daemons,
    error,
    loading,
    { method: 'GET' },
    "Failed to retrieve daemons"
  )
}