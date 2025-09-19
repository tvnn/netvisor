import { writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import type { Service } from './types/base';

export const services = writable<Service[]>([]);

export async function getServices() {
  return await api.request<Service[]>(
    '/services',
    services,
    (services) => services,
    { method: 'GET', },
    true
  )
}

export function getServicesForHost(host_id: string): Service[] {
  let servicesForHost: Service[] = [];
  services.subscribe(services => {
    servicesForHost = services.filter(service => service.host_id === host_id);
  });
  return servicesForHost;
}