// Frontend Service interface that matches the backend Service enum with serde(tag = "type")

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
  service_type: string;
  service_type_config?: Record<string, any>,
  name: string;
  ports: Port[];
  interface_bindings: string[];
}