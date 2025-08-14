import type { NodeStatus } from "$lib/types/nodes";

export const NODE_STATUS_CONFIG = {
  Healthy: {
    display: 'Healthy',
    color: 'text-green-400',
    bgColor: 'bg-green-900/20',
    icon: 'CheckCircle',
    description: 'All tests passing'
  },
  Degraded: {
    display: 'Degraded',
    color: 'text-yellow-400',
    bgColor: 'bg-yellow-900/20',
    icon: 'AlertTriangle',
    description: 'Some non-critical tests failing'
  },
  Failed: {
    display: 'Failed',
    color: 'text-red-400',
    bgColor: 'bg-red-900/20',
    icon: 'XCircle',
    description: 'Critical tests failing'
  },
  Unknown: {
    display: 'Unknown',
    color: 'text-gray-400',
    bgColor: 'bg-gray-900/20',
    icon: 'HelpCircle',
    description: 'No recent test data'
  }
} as const;

export const getNodeStatusDisplayName = (s: NodeStatus) => NODE_STATUS_CONFIG[s]?.display || NODE_STATUS_CONFIG['Unknown'].display;
export const getNodeStatusColor = (s: NodeStatus) => NODE_STATUS_CONFIG[s]?.color || NODE_STATUS_CONFIG['Unknown'].color;
