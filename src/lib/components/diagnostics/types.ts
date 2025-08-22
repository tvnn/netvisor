import type { TestResult } from "../tests/types";

export interface DiagnosticExecutionApi {
  group_id: string;
  trigger_reason: string;
}

export interface DiagnosticExecution {
  id: string;
  group_id: string;
  status: 'Success' | 'Failed' | 'Running';
  trigger_reason: string;
  node_results: NodeResult[];
  generated_remediation_id?: string | null;
  started_at: string;
  completed_at?: string | null;
  created_at: string;
}

export interface NodeResult {
  test_results: TestResult[];
  executed_at: string;
  node_status: string; // "Healthy", etc.
  duration_ms: number;
  
  // These might be missing, we'll need to derive them
  node_id?: string;
  node_name?: string;
  status?: 'Passed' | 'Failed' | 'Skipped' | 'Running';
  sequence?: number;
  error_message?: string;
  completed_at?: string;
  skip_reason?: string;
}

export interface DiagnosticStatistics {
  total_executions: number;
  success_rate: number;
  avg_duration_ms: number;
  recent_failures: number;
}

export interface GroupDiagnosticStatus {
  group_id: string;
  latest_status?: 'Running' | 'Completed' | 'Failed';
  latest_execution_id?: string;
  last_execution_time?: string;
  total_executions: number;
}