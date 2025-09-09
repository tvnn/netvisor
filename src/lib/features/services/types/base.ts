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
  type: string;
  name: string;
  ports: Port[];
  
  // Optional daemon_id for NetvisorDaemon services
  daemon_id?: string;
}

// Helper functions for working with services and the TypeRegistry
export function createDefaultService(serviceType: string, serviceName?: string, defaultPorts?: Port[], defaultEndpoints?: Endpoint[]): Service {
  return {
    type: serviceType,
    name: serviceName || serviceType,
    ports: defaultPorts ? [...defaultPorts] : [],
  };
}

export function getServiceDisplayName(service: Service): string {
  return service.name || service.type;
}

export function formatServicePorts(ports: Port[]): string {
  if (!ports || ports.length === 0) return "No ports";
  
  return ports.map(p => 
    `${p.number}${p.protocol == 'Tcp' ? '/tcp' : '/udp'}`
  ).join(', ');
}