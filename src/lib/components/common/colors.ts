export const COLOR_CONFIG: Record<string, Record<string, string>> = {
    red: {
        textColor: 'text-red-400',
        bgColor: 'bg-red-900/20',
    },
    yellow: {
        textColor: 'text-yellow-400',
        bgColor: 'bg-yellow-900/20',
    },
    blue: {
        textColor: 'text-blue-400',
        bgColor: 'bg-blue-900/20',
    }
} as const;

export const getBgColor = (color: string) => COLOR_CONFIG[color]?.bgColor || 'text-gray-400';
export const getTextColor = (color: string) => COLOR_CONFIG[color]?.textColor || 'bg-gray-900';