// Core type definitions for the Network Diagnostic Tool

// Simplified NetworkNode without type restrictions
export interface NetworkNode {
  id: string;
  name: string;
  domain?: string;
  ip?: string;
  defaultPort?: number;
  path?: string; // For DNS over HTTPS endpoints, service paths, etc.
  description?: string;
  createdAt: string;
  updatedAt: string;
}

export interface CheckConfig {
  target?: string;
  port?: number;
  protocol?: 'http' | 'https';
  timeout?: number;
  domain?: string;
  test_domain?: string;
  service_type?: 'google' | 'cloudflare' | 'pihole' | 'auto';
  path?: string;
  expected_status?: number;
  max_response_time?: number;
  attempts?: number;
  
  // Email server fields
  use_tls?: boolean;
  use_ssl?: boolean;
  
  // SSL certificate fields
  min_days_until_expiry?: number;
  check_chain?: boolean;
  
  // Local network fields
  interface?: string;
  subnet?: string;
  concurrent_scans?: number;
  
  // Protocol-specific fields
  passive_mode?: boolean;
  check_banner?: boolean;
  db_type?: string;
  
  // Performance test fields
  test_duration?: number;
  test_type?: 'download' | 'upload';
  packet_count?: number;
  interval_ms?: number;
  sample_count?: number;
  
  // Advanced test fields
  start_size?: number;
  max_size?: number;
  max_hops?: number;
  timeout_per_hop?: number;
  resolve_hostnames?: boolean;
  port_range?: string;
  scan_type?: 'tcp' | 'udp';
  
  // CDN fields
  expected_region?: string;
  check_headers?: boolean;
  
  // Additional protocol fields
  max_time_drift?: number;
  bind_dn?: string;
  transport?: 'udp' | 'tcp';
}

export interface Check {
  type: string;
  config: CheckConfig;
}

export interface Layer {
  id: string;
  name: string;
  description: string;
  checks: Check[];
  failureActions?: string[];
}

export interface Test {
  id?: string;
  name: string;
  description: string;
  version: string;
  layers: Layer[];
  createdAt?: string;
  updatedAt?: string;
}

export interface CheckResult {
  type: string;
  config: CheckConfig;
  success: boolean;
  message: string;
  error?: string;
  details?: any;
  duration: number;
  startTime: number;
  endTime: number;
}

export interface LayerResult {
  id: string;
  name: string;
  description: string;
  checks: CheckResult[];
  success: boolean;
  startTime: number;
  endTime: number;
  duration: number;
  failureActions?: string[];
}

export interface DiagnosticResults {
  timestamp: string;
  test: string;
  layers: LayerResult[];
  success: boolean;
  totalDuration: number;
}

export interface CheckTypeConfig {
  name: string;
  description: string;
  details: string;
  fields: string[];
  defaults: CheckConfig;
  category: string;
}

export interface NotificationItem {
  id: string;
  message: string;
  type: 'success' | 'error' | 'warning' | 'info';
  duration: number;
  createdAt: number;
}

export interface ModalState {
  isOpen: boolean;
  component: any;
  props: Record<string, any>;
  title: string;
}

export interface LoadingState {
  global: boolean;
  diagnostics: boolean;
  nodes: boolean;
  tests: boolean;
}

export interface ValidationResult {
  valid: boolean;
  errors: string[];
}

// Tauri command types
export interface TauriCommands {
  get_nodes(): Promise<NetworkNode[]>;
  save_node(args: { node: NetworkNode }): Promise<boolean>;
  update_node(args: { id: string; node: NetworkNode }): Promise<boolean>;
  delete_node(args: { id: string }): Promise<boolean>;
  
  get_tests(): Promise<Test[]>;
  save_test(args: { test: Test }): Promise<boolean>;
  update_test(args: { id: string; test: Test }): Promise<boolean>;
  delete_test(args: { id: string }): Promise<boolean>;
  
  run_diagnostics(args: { test: Test }): Promise<DiagnosticResults>;
  get_diagnostic_results(): Promise<DiagnosticResults | null>;
  
  // Basic checks
  check_connectivity(args: CheckConfig): Promise<CheckResult>;
  check_dns_resolution(args: CheckConfig): Promise<CheckResult>;
  check_ping(args: CheckConfig): Promise<CheckResult>;
  
  // Email checks
  check_smtp(args: CheckConfig): Promise<CheckResult>;
  check_imap(args: CheckConfig): Promise<CheckResult>;
  check_pop3(args: CheckConfig): Promise<CheckResult>;
  
  // Security checks
  check_ssl_certificate(args: CheckConfig): Promise<CheckResult>;
  
  // Local checks
  check_dhcp_discovery(args: CheckConfig): Promise<CheckResult>;
  check_subnet_scan(args: CheckConfig): Promise<CheckResult>;
  
  // Protocol checks
  check_ftp(args: CheckConfig): Promise<CheckResult>;
  check_ssh(args: CheckConfig): Promise<CheckResult>;
  check_database(args: CheckConfig): Promise<CheckResult>;
  check_ntp(args: CheckConfig): Promise<CheckResult>;
  check_ldap(args: CheckConfig): Promise<CheckResult>;
  check_sip(args: CheckConfig): Promise<CheckResult>;
  
  // Performance checks
  check_bandwidth(args: CheckConfig): Promise<CheckResult>;
  check_packet_loss(args: CheckConfig): Promise<CheckResult>;
  check_jitter(args: CheckConfig): Promise<CheckResult>;
  
  // Advanced checks
  check_mtu_discovery(args: CheckConfig): Promise<CheckResult>;
  check_traceroute(args: CheckConfig): Promise<CheckResult>;
  check_port_scan(args: CheckConfig): Promise<CheckResult>;
  
  // CDN checks
  check_cdn(args: CheckConfig): Promise<CheckResult>;
  
  export_data(args: { type: string; data: any; filename: string }): Promise<boolean>;
  import_data(args: { type: string; filepath: string }): Promise<{ success: boolean; data: any }>;
}