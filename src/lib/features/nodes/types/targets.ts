

export type NodeTarget = { type: 'IpAddress'; config: IpTargetConfig; } |
{ type: 'Hostname'; config: HostnameTargetConfig; };export interface IpTargetConfig {
  ip: string;
}

export interface HostnameTargetConfig {
  hostname: string;
}

