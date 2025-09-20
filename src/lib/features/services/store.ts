// src/lib/features/services/store.ts
import { writable, derived, get } from 'svelte/store';
import { api } from '../../shared/utils/api';
import type { Endpoint, Port, Service } from './types/base';
import { utcTimeZoneSentinel, uuidv4Sentinel } from '$lib/shared/utils/formatting';

export const services = writable<Service[]>([]);

// Reactive helper to get services for a specific host
export function getServicesForHost(host_id: string) {
  return derived(services, ($services) => 
    $services.filter(service => service.host_id === host_id)
  );
}

// Non-reactive helper for one-time lookups
export function getServicesForHostSync(host_id: string): Service[] {
  return get(services).filter(service => service.host_id === host_id);
}

// Create a new service
export async function createService(service: Omit<Service, 'id' | 'created_at' | 'updated_at'>) {
  return await api.request<Service, Service[]>(
    '/services',
    services,
    (newService: Service, currentServices: Service[]) => [...currentServices, newService],
    { 
      method: 'POST',
      body: JSON.stringify(service)
    }
  );
}

// Update an existing service
export async function updateService(serviceId: string, updates: Partial<Service>) {
  return await api.request<Service, Service[]>(
    `/services/${serviceId}`,
    services,
    (updatedService: Service, currentServices: Service[]) => 
      currentServices.map(s => s.id === serviceId ? updatedService : s),
    { 
      method: 'PUT',
      body: JSON.stringify(updates)
    }
  );
}

// Delete a service
export async function deleteService(serviceId: string) {
  return await api.request<void, Service[]>(
    `/services/${serviceId}`,
    services,
    (_, currentServices: Service[]) => 
      currentServices.filter(s => s.id !== serviceId),
    { method: 'DELETE' }
  );
}

// Get all services
export async function getServices() {
  return await api.request<Service[]>(
    '/services',
    services,
    (services) => services,
    { method: 'GET' },
    true
  );
}

// Bulk operations for host editing
export async function updateHostServices(hostId: string, servicesToUpdate: Service[], servicesToDelete: string[] = []) {
  const promises = [];
  
  // Delete services first
  for (const serviceId of servicesToDelete) {
    promises.push(deleteService(serviceId));
  }
  
  // Update/create services
  for (const service of servicesToUpdate) {
    if (service.id) {
      promises.push(updateService(service.id, service));
    } else {
      promises.push(createService({ ...service, host_id: hostId }));
    }
  }
  
  await Promise.all(promises);
}

// Helper functions for working with services and the TypeRegistry
export function createDefaultService(serviceType: string, host_id: string, serviceName?: string, defaultPorts?: Port[]): Service {
  return {
    id: uuidv4Sentinel,
    created_at: utcTimeZoneSentinel,
    updated_at: utcTimeZoneSentinel,
    host_id,
    service_type: {type: serviceType},
    name: serviceName || serviceType,
    ports: defaultPorts ? [...defaultPorts] : [],
    interface_bindings: []
  };
}

export function getServiceDisplayName(service: Service): string {
  return service.name || service.service_type.type;
}

export function formatServicePorts(ports: Port[]): string {
  if (!ports || ports.length === 0) return "No ports";
  
  return ports.map(p => 
    `${p.number}${p.protocol == 'Tcp' ? '/tcp' : '/udp'}`
  ).join(', ');
}