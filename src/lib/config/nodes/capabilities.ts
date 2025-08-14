import type { NodeCapability } from "./types";

export const CAPABILITY_CONFIG = {
  SshAccess: {
    display: 'SSH Access',
    description: 'Remote command-line access for management and troubleshooting',
    category: 'Remote Access',
    icon: 'Terminal',
    color: 'text-green-400'
  },
  HttpService: {
    display: 'HTTP Service',
    description: 'Web interface or API accessible over HTTP',
    category: 'Web Services',
    icon: 'Globe',
    color: 'text-blue-400'
  },
  HttpsService: {
    display: 'HTTPS Service',
    description: 'Secure web interface or API accessible over HTTPS',
    category: 'Web Services', 
    icon: 'Shield',
    color: 'text-blue-400'
  },
  VpnService: {
    display: 'VPN Service',
    description: 'VPN server for secure remote access',
    category: 'Security',
    icon: 'Lock',
    color: 'text-orange-400'
  },
  DnsService: {
    display: 'DNS Service',
    description: 'Domain name resolution service',
    category: 'Network Infrastructure',
    icon: 'Search',
    color: 'text-purple-400'
  },
  DhcpService: {
    display: 'DHCP Service',
    description: 'Automatic IP address assignment for network devices',
    category: 'Network Infrastructure',
    icon: 'Network',
    color: 'text-cyan-400'
  }
} as const;

// Utility functions
export const getCapabilityDisplay = (capability: NodeCapability) => CAPABILITY_CONFIG[capability].display;
export const getCapabilityDescription = (capability: NodeCapability) => CAPABILITY_CONFIG[capability].description;
export const getCapabilityCategory = (capability: NodeCapability) => CAPABILITY_CONFIG[capability].category;
export const getCapabilityIcon = (capability: NodeCapability) => CAPABILITY_CONFIG[capability].icon;
export const getCapabilityColor = (capability: NodeCapability) => CAPABILITY_CONFIG[capability].color;

// Get all capabilities
export const getAllCapabilities = (): NodeCapability[] => 
  Object.keys(CAPABILITY_CONFIG) as NodeCapability[];

// Get capabilities by category
export const getCapabilitiesByCategory = (category: string): NodeCapability[] =>
  Object.entries(CAPABILITY_CONFIG)
    .filter(([_, config]) => config.category === category)
    .map(([capability, _]) => capability as NodeCapability);

// Get all categories
export const getCapabilityCategories = (): string[] =>
  [...new Set(Object.values(CAPABILITY_CONFIG).map(config => config.category))];