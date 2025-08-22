import { writable } from 'svelte/store';
import type { DiagnosticExecution } from './types';
import { api } from '../../api/client';
import { AlertTriangle, CheckCircle, Clock, Loader2 } from 'lucide-svelte';

export const diagnosticExecutions = writable<DiagnosticExecution[]>([]);
export const loading = writable(false);
export const error = writable<string | null>(null);

export const diagnosticsActionsStore = writable({
  handleViewDetails: null as ((execution: DiagnosticExecution) => void) | null,
  handleDelete: null as ((execution: DiagnosticExecution) => void) | null,
  deletingId: null as string | null,
});

export const diagnosticsActions = {

  async loadExecutions() {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.getDiagnostics();
      if (response.success && response.data) {
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