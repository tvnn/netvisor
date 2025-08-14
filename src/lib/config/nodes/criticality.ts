import type { TestCriticality } from "$lib/types/nodes";

export const CRITICALITY_CONFIG = {
  Critical: {
    display: 'Critical',
    color: 'text-red-400',
    bgColor: 'bg-red-900/20',
    description: 'Failure results in node status: Failed',
    priority: 1
  },
  Important: {
    display: 'Important',
    color: 'text-yellow-400',
    bgColor: 'bg-yellow-900/20',
    description: 'Failure results in node status: Degraded',
    priority: 2
  },
  Informational: {
    display: 'Informational',
    color: 'text-blue-400',
    bgColor: 'bg-blue-900/20',
    description: 'Failure is logged but does not affect node status',
    priority: 3
  }
} as const;

export const getCriticalityDisplay = (c: TestCriticality) => CRITICALITY_CONFIG[c].display;
export const getCriticalityColor = (c: TestCriticality) => CRITICALITY_CONFIG[c].color;
export const getCriticalityBgColor = (c: TestCriticality) => CRITICALITY_CONFIG[c].bgColor;
