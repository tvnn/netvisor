import type { NodeType } from "$lib/types/nodes";
import { Router, Network, Wifi, Shield, Lock, Search, Globe, Play, Database, HardDrive, Monitor, Cpu, Printer, Camera, HelpCircle } from 'lucide-svelte';

export const NODE_TYPE_CONFIG = {
  Router: {
    display: 'Router',
    icon: Router,
    category: 'Infrastructure',
    color: 'text-blue-400'
  },
  Switch: {
    display: 'Switch',
    icon: Network,
    category: 'Infrastructure',
    color: 'text-blue-400'
  },
  AccessPoint: {
    display: 'Access Point',
    icon: Wifi,
    category: 'Infrastructure',
    color: 'text-blue-400'
  },
  Firewall: {
    display: 'Firewall',
    icon: Shield,
    category: 'Security',
    color: 'text-red-400'
  },
  VpnServer: {
    display: 'VPN Server',
    icon: Lock,
    category: 'Security',
    color: 'text-orange-400'
  },
  DnsServer: {
    display: 'DNS Server',
    icon: Search,
    category: 'Server',
    color: 'text-purple-400'
  },
  WebServer: {
    display: 'Web Server',
    icon: Globe,
    category: 'Server',
    color: 'text-green-400'
  },
  DatabaseServer: {
    display: 'Database Server',
    icon: Database,
    category: 'Server',
    color: 'text-yellow-400'
  },
  MediaServer: {
    display: 'Media Server',
    icon: Play,
    category: 'Server',
    color: 'text-pink-400'
  },
  NasDevice: {
    display: 'NAS Device',
    icon: HardDrive,
    category: 'Storage',
    color: 'text-gray-400'
  },
  Workstation: {
    display: 'Workstation',
    icon: Monitor,
    category: 'Endpoint',
    color: 'text-gray-400'
  },
  IotDevice: {
    display: 'IoT Device',
    icon: Cpu,
    category: 'Endpoint',
    color: 'text-teal-400'
  },
  Printer: {
    display: 'Printer',
    icon: Printer,
    category: 'Endpoint',
    color: 'text-gray-400'
  },
  Camera: {
    display: 'Camera',
    icon: Camera,
    category: 'Endpoint',
    color: 'text-gray-400'
  },
  UnknownDevice: {
    display: 'Unknown Device',
    icon: HelpCircle,
    category: 'Unknown',
    color: 'text-gray-500'
  }
} as const;

export const getNodeTypeDisplay = (type: NodeType) => NODE_TYPE_CONFIG[type]?.display || NODE_TYPE_CONFIG['UnknownDevice'].display;
export const getNodeTypeIcon = (type: NodeType) => NODE_TYPE_CONFIG[type]?.icon || NODE_TYPE_CONFIG['UnknownDevice'].icon;
export const getNodeTypeCategory = (type: NodeType) => NODE_TYPE_CONFIG[type].category || NODE_TYPE_CONFIG['UnknownDevice'].category;
export const getNodeTypeColor = (type: NodeType) => NODE_TYPE_CONFIG[type].color || NODE_TYPE_CONFIG['UnknownDevice'].color;

export const getNodeTypes = () => Object.keys(NODE_TYPE_CONFIG) as NodeType[];

export const getNodeTypesByCategory = (category: string) => Object.entries(NODE_TYPE_CONFIG)
  .filter(([_, config]) => config.category === category)
  .map(([type, _]) => type as NodeType);
