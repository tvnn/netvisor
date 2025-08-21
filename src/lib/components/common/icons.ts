import { Database, Globe, Router, Server, Shield, Wifi } from 'lucide-svelte';

const ICON_CONFIG: Record<string, any> = {
    'Globe': Globe,
    'Server': Server,
    'Router': Router,
    'Wifi': Wifi,
    'Database': Database,
    'Shield': Shield,
} as const;

export const getIcon = (icon: string) => ICON_CONFIG[icon] || Server;