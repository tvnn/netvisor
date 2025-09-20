import type { Port, Service } from "$lib/features/services/types/base";

export interface Host {
  id: string;
  created_at: string;
  updated_at: string;
  name: string;
  description: string;
  hostname: string;
  target: HostTarget;
  services: string[];
  interfaces: Interface[];
  groups: string[];
}

export interface Interface {
    id: string,
    subnet_id: string,
    name: string,
    ip_address?: string,
    mac_address?: string
    is_primary: boolean
}

export type HostTarget = 
  | { type: 'Interface'; config: string }  // UUID of interface
  | { type: 'ExternalIp'; config: string }  // IP Address
  | { type: 'Hostname' };

// For backwards compatibility during transition
export interface IpTargetConfig {
  ip: string;
}

export interface HostnameTargetConfig {
  hostname: string;
}