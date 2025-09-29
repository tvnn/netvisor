import type { Service } from "$lib/features/services/types/base";

export interface HostWithServicesRequest {
  host: Host,
  services: Service[]
}

export interface Host {
  id: string;
  created_at: string;
  updated_at: string;
  name: string;
  description: string;
  hostname: string;
  target: HostTarget;
  services: string[];
  ports: Port[];
  interfaces: Interface[];
}

export interface Interface {
    id: string,
    subnet_id: string,
    name: string,
    ip_address?: string,
    mac_address?: string
}

export type HostTarget = 
  | { type: 'ServiceBinding'; config: ServiceBinding }
  | { type: 'None' }
  | { type: 'Hostname' };

// For backwards compatibility during transition
export interface IpTargetConfig {
  ip: string;
}

export interface HostnameTargetConfig {
  hostname: string;
}
export interface ServiceBinding {
  service_id: string;
  interface_id: string;
  port_id: string;
}

export interface Port {
  number: number;
  protocol: string;
  id: string;
  type: string;
}

