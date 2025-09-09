import type { Port, Service } from "$lib/features/services/types/base";
import type { HostTarget } from "./targets";

export interface Host {
  id: string;
  created_at: string;
  updated_at: string;
  last_seen: string;
  name: string;
  description: string;
  hostname: string;
  target: HostTarget;
  services: Service[];
  open_ports: Port[]
  subnets: HostSubnetMembership[];
  groups: string[];
}

export interface HostSubnetMembership {
    subnet_id: string,
    ip_address?: string,
    mac_address?: string
    default: boolean
}