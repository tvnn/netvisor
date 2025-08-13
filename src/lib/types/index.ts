// Basic types that mirror the Rust backend

export type NodeType = 
  | 'Router' | 'Switch' | 'AccessPoint' | 'Firewall'
  | 'WebServer' | 'DatabaseServer' | 'MediaServer' 
  | 'DnsServer' | 'VpnServer' | 'NasDevice'
  | 'Workstation' | 'IotDevice' | 'Printer' | 'Camera'
  | 'UnknownDevice';

export type NodeCapability = 
  | 'SshAccess' | 'RdpAccess' | 'VncAccess'
  | 'HttpService' | 'HttpsService'
  | 'MysqlService' | 'PostgresService' | 'MongoService' | 'RedisService'
  | 'DnsService' | 'EmailService' | 'FtpService';

export type NodeStatus = 'Healthy' | 'Degraded' | 'Failed' | 'Unknown';

export type TestType = 
  | 'Connectivity' | 'DirectIp' | 'Ping' | 'WellknownIp'
  | 'DnsResolution' | 'DnsOverHttps'
  | 'VpnConnectivity' | 'VpnTunnel'
  | 'ServiceHealth' | 'DaemonCommand' | 'SshScript';

export type TestCriticality = 'Critical' | 'Important' | 'Informational';

export interface Node {
  id: string;
  name: string;
  domain?: string;
  ip?: string;
  port?: number;
  path?: string;
  description?: string;
  created_at: string;
  updated_at: string;
  node_type?: NodeType;
  capabilities: NodeCapability[];
  assigned_tests: AssignedTest[];
  monitoring_enabled: boolean;
  node_groups: string[];
  current_status: NodeStatus;
  subnet_membership: string[];
  last_seen?: string;
}

export interface AssignedTest {
  test_type: TestType;
  test_config: any; // Simplified for now
  monitor_interval_minutes?: number;
  enabled: boolean;
  criticality: TestCriticality;
}

export interface NodeGroup {
  id: string;
  name: string;
  description: string;
  node_sequence: string[];
  auto_diagnostic_enabled: boolean;
}

// Helper functions
export const getNodeTypeDisplayName = (nodeType: NodeType): string => {
  const displayNames: Record<NodeType, string> = {
    Router: 'Router', Switch: 'Switch', AccessPoint: 'Access Point', Firewall: 'Firewall',
    WebServer: 'Web Server', DatabaseServer: 'Database Server', MediaServer: 'Media Server',
    DnsServer: 'DNS Server', VpnServer: 'VPN Server', NasDevice: 'NAS Device',
    Workstation: 'Workstation', IotDevice: 'IoT Device', Printer: 'Printer', Camera: 'Camera',
    UnknownDevice: 'Unknown Device'
  };
  return displayNames[nodeType];
};

export const getStatusColor = (status: NodeStatus): string => {
  const colors = { Healthy: 'green', Degraded: 'yellow', Failed: 'red', Unknown: 'gray' };
  return colors[status];
};

export const getTestTypeDisplayName = (testType: TestType): string => {
  const displayNames: Record<TestType, string> = {
    Connectivity: 'Connectivity', DirectIp: 'Direct IP', Ping: 'Ping', WellknownIp: 'Well-known IP',
    DnsResolution: 'DNS Resolution', DnsOverHttps: 'DNS over HTTPS',
    VpnConnectivity: 'VPN Connectivity', VpnTunnel: 'VPN Tunnel',
    ServiceHealth: 'Service Health', DaemonCommand: 'Daemon Command', SshScript: 'SSH Script'
  };
  return displayNames[testType];
};