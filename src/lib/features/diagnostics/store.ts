import { writable } from 'svelte/store';
import type { DiagnosticExecution, DiagnosticExecutionBase } from "./types/base";
import { api } from '../../shared/utils/api';
import { AlertTriangle, CheckCircle, Clock, Loader2 } from 'lucide-svelte';

export const diagnosticExecutions = writable<DiagnosticExecution[]>([]);

export const diagnosticsTableActionsStore = writable({
  handleViewDetails: null as ((execution: DiagnosticExecution) => void) | null,
  handleDelete: null as ((execution: DiagnosticExecution) => void) | null,
  deletingId: null as string | null,
});

export async function getDiagnosticExecutions() {
  return await api.request<DiagnosticExecution[]>(
    '/diagnostics',
    diagnosticExecutions,
    (diagnostics) => diagnostics,
    { method: 'GET' },
  )
}

export async function deleteDiagnosticExecutions(id: string) {
  await api.request<void, DiagnosticExecution[]>(
    `/diagnostics/${id}`,
    null,
    (_, current) => current.filter(d => d.id !== id),
    {},
  )
}

export async function executeDiagnostics(group_id: string, data: DiagnosticExecutionBase) {
  api.request<DiagnosticExecution, DiagnosticExecution[]>(
    `/diagnostics/execute/${group_id}`,
    diagnosticExecutions,
    (execution, current) => [...current, execution],
    { method: 'POST', body: JSON.stringify(data) },
  )
}