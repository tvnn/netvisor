import { AlertTriangle, CheckCircle, CircleQuestionMark, CircleX, Database, Globe, Loader2, LucideCircleQuestionMark, RectangleGoggles, Router, Search, Server, Shield, Wifi } from 'lucide-svelte';

const ICON_CONFIG: Record<string, any> = {
    'Globe': Globe,
    'Server': Server,
    'Router': Router,
    'Wifi': Wifi,
    'Database': Database,
    'Shield': Shield,
    'Search': Search,
    "Loader2": Loader2,
    "CheckCircle": CheckCircle,
    "AlertTriangle": AlertTriangle,
    "CircleX": CircleX,
    "CircleQuestionMark": CircleQuestionMark,
} as const;

export const getIcon = (icon: string) => ICON_CONFIG[icon] || LucideCircleQuestionMark;