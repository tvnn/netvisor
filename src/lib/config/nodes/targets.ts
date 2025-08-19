import { ApplicationProtocol, type HostnameTargetConfig, type Ipv4NodeTargetConfig, type Ipv6NodeTargetConfig, type NodeTarget, type NodeTargetType, type ServiceTargetConfig } from "$lib/types/nodes";

interface NodeTargetTypeConfig {
    display: string,
    description: string,
    defaultConfig: Record<string, any>
}

export const NODE_TARGET_TYPE_CONFIG: Record<string, NodeTargetTypeConfig> = {
    Ipv4Address:{
        display: "IPv4 Address",
        description: "Direct connection using IPv4 address (bypasses DNS)",
        defaultConfig: {
            ip: "127.0.0.1",
            port: 80
        }
    },
    Ipv6Address:{
        display: "IPv6 Address",
        description: "",
        defaultConfig: {
            ip: "127.0.0.1",
            port: 80
        }
    },
    Hostname:{
        display: "Hostname",
        description: "Connect using domain name or hostname (requires DNS resolution)",
        defaultConfig: {
            hostname: "127.0.0.1",
            port: 80
        }
    },
    Service:{
        display: "Service URL",
        description: "Full service endpoint with protocol and path (for web services)",
        defaultConfig: {
            protocol: ApplicationProtocol.Https,
            hostname: "127.0.0.1",
            port: 80,
            path: '/'
        }
    }
} as const;

export const getNodeTargetTypeDisplayName = (targetType: NodeTargetType) => NODE_TARGET_TYPE_CONFIG[targetType].display;
export const getNodeTargetTypeDescription = (targetType: NodeTargetType) => NODE_TARGET_TYPE_CONFIG[targetType].description;

export const getNodeTargetTypes = (): NodeTargetType[] => Object.keys(NODE_TARGET_TYPE_CONFIG) as NodeTargetType[];

export function getNodeTargetTypeDefaultConfig(targetType: NodeTargetType): any {

    let defaultConfig = NODE_TARGET_TYPE_CONFIG[targetType].defaultConfig

    switch (targetType) {
        case 'Ipv4Address':
            return defaultConfig as Ipv4NodeTargetConfig;
        case 'Ipv6Address':
            return defaultConfig as Ipv6NodeTargetConfig;
        case 'Hostname':
            return defaultConfig as HostnameTargetConfig;
        case 'Service':
            return defaultConfig as ServiceTargetConfig;
    }
}

// Helper functions for target handling
export function getNodeTargetString(target: NodeTarget): string {
  switch (target.type) {
    case 'Ipv4Address':
    case 'Ipv6Address':
      return target.config.ip + (target.config.port ? `:${target.config.port}` : '');
    case 'Hostname':
      return target.config.hostname + (target.config.port ? `:${target.config.port}` : '');
    case 'Service':
      const base = `${target.config.protocol}://${target.config.hostname}`;
      const port = target.config.port ? `:${target.config.port}` : '';
      const path = target.config.path || '';
      return base + port + path;
    default:
      return 'Unknown target';
  }
}

export function validateTarget(target: NodeTarget): string[] {
  const errors: string[] = [];
  
  switch (target.type) {
    case 'Ipv4Address':
    case 'Ipv6Address':
      if (!target.config.ip) {
        errors.push('IP address is required');
      }
      break;
    case 'Hostname':
      if (!target.config.hostname) {
        errors.push('Hostname is required');
      }
      break;
    case 'Service':
      if (!target.config.protocol) {
        errors.push('Protocol is required');
      }
      if (!target.config.hostname) {
        errors.push('Hostname is required');
      }
      break;
  }
  
  if (target.config.port && (target.config.port < 1 || target.config.port > 65535)) {
    errors.push('Port must be between 1 and 65535');
  }
  
  return errors;
}

