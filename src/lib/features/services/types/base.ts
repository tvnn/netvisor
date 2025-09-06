// Frontend Service interface that matches the backend Service enum with serde(tag = "type")

export interface Port {
  number: number;
  udp: boolean;
  tcp: boolean;
}

export interface Endpoint {
  url?: string;
  method?: string;
  protocol?: string;
  ip?: string;
  port?: Port;
  path?: string;
}

export interface Service {
  // Service type (automatically added by serde tag)
  type: string;
  
  // Common fields shared by all service variants
  confirmed: boolean;
  name: string;
  ports: Port[];
  endpoints: Endpoint[];
  
  // Optional daemon_id for NetvisorDaemon services
  daemon_id?: string;
}

// Helper functions for working with services and the TypeRegistry
export function createDefaultService(serviceType: string, serviceName?: string, defaultPorts?: Port[], defaultEndpoints?: Endpoint[]): Service {
  return {
    type: serviceType,
    confirmed: false,
    name: serviceName || serviceType,
    ports: defaultPorts ? [...defaultPorts] : [],
    endpoints: defaultEndpoints ? [...defaultEndpoints] : []
  };
}

export function getServiceDisplayName(service: Service): string {
  return service.name || service.type;
}

export function formatServicePorts(ports: Port[]): string {
  if (!ports || ports.length === 0) return "No ports";
  
  return ports.map(p => 
    `${p.number}${p.tcp && p.udp ? '/tcp+udp' : p.tcp ? '/tcp' : '/udp'}`
  ).join(', ');
}