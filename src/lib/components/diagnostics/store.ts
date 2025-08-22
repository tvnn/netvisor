import { writable } from 'svelte/store';
import type { DiagnosticExecution } from './types';
import { api } from '../../api/client';
import { AlertTriangle, CheckCircle, Clock, Loader2 } from 'lucide-svelte';

export const diagnosticExecutions = writable<DiagnosticExecution[]>([]);
export const loading = writable(false);
export const error = writable<string | null>(null);

export const diagnosticsActions = {
  async loadExecutions() {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.getDiagnostics();
      console.log('API Response:', response); // Add this
      if (response.success && response.data) {
        console.log('Executions data:', response.data.executions); // Add this
        diagnosticExecutions.set(response.data.executions);
      } else {
        error.set(response.error || 'Failed to load diagnostics');
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to load diagnostics:', err);
    } finally {
      loading.set(false);
    }
  },

  async deleteExecution(id: string): Promise<boolean> {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.deleteDiagnostic(id);
      if (response.success) {
        diagnosticExecutions.update(current => 
          current.filter(execution => execution.id !== id)
        );
        return true;
      } else {
        error.set(response.error || 'Failed to delete diagnostic');
        return false;
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to delete diagnostic:', err);
      return false;
    } finally {
      loading.set(false);
    }
  },

  clearError() {
    error.set(null);
  }
};

export function formatDuration(startTime: string, endTime?: string) {
  if (!startTime) return '';
  
  const start = new Date(startTime);
  const end = endTime ? new Date(endTime) : new Date();
  const durationMs = end.getTime() - start.getTime();
  
  if (durationMs < 1000) return '<1s';
  if (durationMs < 60000) return `${Math.round(durationMs / 1000)}s`;
  if (durationMs < 3600000) return `${Math.round(durationMs / 60000)}m`;
  return `${Math.round(durationMs / 3600000)}h`;
}

export function formatTimestamp(timestamp: string) {
  if (!timestamp) return 'Unknown';
  try {
    return new Date(timestamp).toLocaleString();
  } catch {
    return 'Invalid date';
  }
}

// Convert API status to display status
export function getDisplayStatus(apiStatus: string): string {
  switch (apiStatus) {
    case 'Success': return 'Completed';
    case 'Failed': return 'Failed';
    case 'Running': return 'Running';
    default: return apiStatus;
  }
}

export function getStatusIcon(status: string) {
  switch (status) {
    case 'Completed': return CheckCircle;
    case 'Failed': return AlertTriangle;
    case 'Running': return Loader2;
    default: return Clock;
  }
}

export function getStatusClass(status: string) {
  switch (status) {
    case 'Completed': return 'text-green-400';
    case 'Failed': return 'text-red-400';
    case 'Running': return 'text-blue-400 animate-spin';
    default: return 'text-gray-400';
  }
}

export function getStatusBadgeClass(status: string) {
  switch (status) {
    case 'Completed': return 'bg-green-500/20 text-green-400 border-green-500/30';
    case 'Failed': return 'bg-red-500/20 text-red-400 border-red-500/30';
    case 'Running': return 'bg-blue-500/20 text-blue-400 border-blue-500/30';
    default: return 'bg-gray-500/20 text-gray-400 border-gray-500/30';
  }
}

export function getNodeBackgroundClass(status: string) {
  // Since API uses node_status like "Healthy", derive status from test results
  if (!status || status === 'Healthy') {
    return 'bg-gray-800 border-gray-700';
  }
  switch (status) {
    case 'Failed': return 'bg-red-500/10 border-red-500/30';
    case 'Running': return 'bg-blue-500/10 border-blue-500/30';
    case 'Skipped': return 'bg-gray-500/10 border-gray-600';
    default: return 'bg-gray-800 border-gray-700';
  }
}