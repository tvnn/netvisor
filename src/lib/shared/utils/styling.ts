// src/lib/shared/utils/styling.ts
import * as LucideIcons from 'lucide-svelte';

export interface ColorStyle {
    text: string,
    bg: string,
    border: string,
    icon: string
}

// Unified color helper - works everywhere!
export function createColorHelper(colorName: string | null): ColorStyle {
  if (!colorName || colorName == null) {
    return {
      text: 'text-gray-400',
      bg: 'bg-gray-900/20 border-gray-600',
      border: 'border-gray-600',
      icon: 'text-gray-400'
    };
  }
  
  // Handle common color names from backend
  const normalizedColor = colorName.toLowerCase().trim();
  
  // Map backend color names to Tailwind classes
  const colorMap: Record<string, ColorStyle> = {
    'red': { text: 'text-red-400', bg: 'bg-red-900/20 border-red-600', border: 'border-red-600', icon: 'text-red-400' },
    'green': { text: 'text-green-400', bg: 'bg-green-900/20 border-green-600', border: 'border-green-600', icon: 'text-green-400' },
    'blue': { text: 'text-blue-400', bg: 'bg-blue-900/20 border-blue-600', border: 'border-blue-600', icon: 'text-blue-400' },
    'yellow': { text: 'text-yellow-400', bg: 'bg-yellow-900/20 border-yellow-600', border: 'border-yellow-600', icon: 'text-yellow-400' },
    'purple': { text: 'text-purple-400', bg: 'bg-purple-900/20 border-purple-600', border: 'border-purple-600', icon: 'text-purple-400' },
    'indigo': { text: 'text-indigo-400', bg: 'bg-indigo-900/20 border-indigo-600', border: 'border-indigo-600', icon: 'text-indigo-400' },
    'pink': { text: 'text-pink-400', bg: 'bg-pink-900/20 border-pink-600', border: 'border-pink-600', icon: 'text-pink-400' },
    'gray': { text: 'text-gray-400', bg: 'bg-gray-900/20 border-gray-600', border: 'border-gray-600', icon: 'text-gray-400' },
    'orange': { text: 'text-orange-400', bg: 'bg-orange-900/20 border-orange-600', border: 'border-orange-600', icon: 'text-orange-400' },
    'teal': { text: 'text-teal-400', bg: 'bg-teal-900/20 border-teal-600', border: 'border-teal-600', icon: 'text-teal-400' },
    'cyan': { text: 'text-cyan-400', bg: 'bg-cyan-900/20 border-cyan-600', border: 'border-cyan-600', icon: 'text-cyan-400' },
    'emerald': { text: 'text-emerald-400', bg: 'bg-emerald-900/20 border-emerald-600', border: 'border-emerald-600', icon: 'text-emerald-400' },
    'rose': { text: 'text-rose-400', bg: 'bg-rose-900/20 border-rose-600', border: 'border-rose-600', icon: 'text-rose-400' }
  };
  
  return colorMap[normalizedColor] || colorMap['gray'];
}

// Icon helper that converts string to component
export function createIconComponent(iconName: string | null) {
  if (!iconName || iconName == null) return LucideIcons.HelpCircle;
  
  // Convert kebab-case to PascalCase for Lucide component names
  const componentName = iconName
    .split('-')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join('');
  
  // Return the component or fallback
  return (LucideIcons as any)[componentName] || LucideIcons.HelpCircle;
}

// Convenience wrapper that returns both color and icon
export function createStyle(color: string | null, icon: string | null) {
  return {
    colors: createColorHelper(color),
    IconComponent: createIconComponent(icon),
    iconName: icon
  };
}