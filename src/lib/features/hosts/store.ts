import { get, writable } from 'svelte/store';
import type { Host, HostTarget, HostWithServicesRequest, Interface } from "./types/base";
import { api } from '../../shared/utils/api';
import { pushInfo, pushSuccess, pushWarning } from '$lib/shared/stores/feedback';
import { utcTimeZoneSentinel, uuidv4Sentinel } from '$lib/shared/utils/formatting';
import { getServices } from '../services/store';

export const hosts = writable<Host[]>([]);
export const polling = writable(false);

export async function getHosts() {
  return await api.request<Host[]>(
    '/hosts',
    hosts,
    (hosts) => hosts,
    { method: 'GET', },
    true
  )
}

export async function createHost(data: HostWithServicesRequest) {
  const result = await api.request<Host, Host[]>(
    '/hosts',
    hosts,
    (host, current) => [...current, host],
    { method: 'POST', body: JSON.stringify(data)},
  )

  return result
}

export async function updateHost(data: HostWithServicesRequest) {
  return await api.request<Host, Host[]>(
    `/hosts`,
    hosts,
    (updatedHost, current) => {
      return current.map(n => n.id === data.host.id ? updatedHost : n)
    },
    { method: 'PUT', body: JSON.stringify(data)},
  )
}

export async function deleteHost(id: string) {
  return await api.request<void, Host[]>(
    `/hosts/${id}`,
    hosts,
    (_, current) => current.filter(g => g.id !== id),
    { method: 'DELETE'},
  )
}

export async function consolidateHosts(destination_host_id: string, other_host_id: string) {

  let other_host_name = getHostFromId(other_host_id)?.name;

  return await api.request<Host, Host[]>(
    `/hosts/${destination_host_id}/consolidate/${other_host_id}`,
    hosts,
    (updatedHost, current) => {
      current = current.filter(g => g.id !== other_host_id);
      pushSuccess(`Consolidated host "${other_host_name}" into host "${updatedHost.name}"`)
      return current.map(h => h.id == destination_host_id ? updatedHost : h);
    },
    { method: 'PUT'},
  )
}

export function createEmptyHostFormData(): Host {
  return {
    id: uuidv4Sentinel,
    created_at: utcTimeZoneSentinel,
    updated_at: utcTimeZoneSentinel,
    name: '',
    description: '',
    hostname: '',
    target: {
      type: 'Hostname',
    },
    services: [],
    interfaces: [],
  };
}

export function getHostTargetString(host: Host): string {
  switch (host.target.type) {
    case 'Interface':
      let iface = getInterfaceFromId(host.target.config);
      return iface ? iface.ip_address || iface.mac_address || iface.name : 'Unknown interface';
    case 'ExternalIp':
      return host.target.config || 'Unknown external IP';
    case 'Hostname':
      return host.hostname;
    default:
      return 'Unknown target';
  }
}

export function getHostFromId(id: string): Host | undefined {
  return get(hosts).find(h => h.id == id)
}

export function getInterfaceFromId(id: string): Interface | undefined {
  for (const host of get(hosts)) {
    const iface = host.interfaces.find(i => i.id == id);
    if (iface != undefined) {
      return iface;
    }
  }
  return undefined;
}
