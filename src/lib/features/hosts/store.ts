import { writable } from 'svelte/store';
import type { Host } from "./types/base";
import { api } from '../../shared/utils/api';
import type { HostTarget } from './types/targets';
import { pushInfo, pushWarning } from '$lib/shared/stores/feedback';
import { utcTimeZoneSentinel, uuidv4Sentinel } from '$lib/shared/utils/formatting';

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

export async function createHost(data: Host) {
  return await api.request<Host, Host[]>(
    '/hosts',
    hosts,
    (host, current) => [...current, host],
    { method: 'POST', body: JSON.stringify(data)},
  )
}

interface UpdateHostResponse {
  host: Host,
  // capability_test_changes: Record<string, HostCapabilityTestChange>,
  subnet_changes: HostSubnetRelationshipChange
}

// interface HostCapabilityTestChange {
//     newly_compatible: string[], 
//     incompatible: string[]
// }

interface HostSubnetRelationshipChange {
  new_gateway: Subnet[],
  no_longer_gateway: Subnet[],
  new_dns_resolver: Subnet[],
  no_longer_dns_resolver: Subnet[]
}

export async function updateHost(data: Host) {
  return await api.request<UpdateHostResponse, Host[]>(
    `/hosts/${data.id}`,
    hosts,
    (updatedHostResponse, current) => {
      const updatedHost = updatedHostResponse.host;

      // Object.keys(updatedHostResponse.capability_test_changes).forEach(cap => {
      //   let incompatible = updatedHostResponse.capability_test_changes[cap].incompatible.map(i => testTypes.getDisplay(i))
      //   let newly_compatible = updatedHostResponse.capability_test_changes[cap].newly_compatible.map(n => testTypes.getDisplay(n))
      //   incompatible.length > 0 ? pushWarning(`The following tests are no longer compatible with host "${updatedHost.name}" and have been removed: ${incompatible.join(", ")}`) : null
      //   newly_compatible.length > 0 ? pushInfo(`The following tests are now compatible with host "${updatedHost.name}" and have been added: ${newly_compatible.join(", ")}`) : null
      // })

      if (updatedHostResponse.subnet_changes.new_dns_resolver.length > 0) {
        pushInfo(`The following subnets now have host "${updatedHost.name}" set as a DNS resolver: ${
          updatedHostResponse.subnet_changes.new_dns_resolver.map(d => `${d.name} (${d.cidr})`).join(", ")
        }`)
      }

      if (updatedHostResponse.subnet_changes.new_gateway.length > 0) {
        pushInfo(`The following subnets now have host "${updatedHost.name}" set as a gateway: ${
          updatedHostResponse.subnet_changes.new_gateway.map(d => `${d.name} (${d.cidr})`).join(", ")
        }`)
      }

      if (updatedHostResponse.subnet_changes.no_longer_dns_resolver.length > 0) {
        pushWarning(`The following subnets no longer have host "${updatedHost.name}" set as a gateway: ${
          updatedHostResponse.subnet_changes.no_longer_dns_resolver.map(d => `${d.name} (${d.cidr})`).join(", ")
        }`)
      }

      if (updatedHostResponse.subnet_changes.no_longer_gateway.length > 0) {
        pushWarning(`The following subnets no longer have host "${updatedHost.name}" set as a gateway: ${
          updatedHostResponse.subnet_changes.no_longer_gateway.map(d => `${d.name} (${d.cidr})`).join(", ")
        }`)
      }

      return current.map(n => n.id === data.id ? updatedHost : n)
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


export function createEmptyHostFormData(): Host {
  return {
    id: uuidv4Sentinel,
    created_at: utcTimeZoneSentinel,
    updated_at: utcTimeZoneSentinel,
    name: '',
    description: '',
    hostname: '',
    target: {
      type: 'IpAddress',
      config: {
        ip: '127.0.0.1',
      },
    },
    services: [],
    open_ports: [],
    subnets: [],
    last_seen: utcTimeZoneSentinel,
    groups: [],
  };
}

export function getHostTargetString(target: HostTarget): string {
  switch (target.type) {
    case 'IpAddress':
      return target.config.ip;
    case 'Hostname':
      return target.config.hostname;
    default:
      return 'Unknown target';
  }
}
