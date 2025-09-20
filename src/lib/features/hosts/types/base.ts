import type { Port, Service } from "$lib/features/services/types/base";
import type { HostTarget } from "./targets";

export interface Host {
  id: string;
  created_at: string;
  updated_at: string;
  name: string;
  description: string;
  hostname: string;
  target: HostTarget;
  services: string[];
  open_ports: Port[]
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
