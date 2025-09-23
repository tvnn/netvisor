export interface Port {
  number: number;
  protocol: string;
}

export interface Endpoint {
  protocol?: string;
  ip?: string;
  port?: Port;
  path?: string;
}

export interface Service {
  id: string,
  created_at: string,
  updated_at: string,
  host_id: string;
  service_definition: string;
  name: string;
  ports: Port[];
  interface_bindings: string[];
  groups: string[];
}