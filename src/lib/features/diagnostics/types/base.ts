export interface DiagnosticExecutionBase {
  group_id: string;
  trigger_reason: string;
}

export interface DiagnosticExecution extends DiagnosticExecutionBase {
  id: string;
  status: string;
  node_results: NodeResult[];
  generated_remediation_id?: string | null;
  started_at: string;
  completed_at?: string | null;
  created_at: string;
}

export interface NodeResult {
  test_results: TestResult[];
  executed_at: string;
  node_status: string;
  duration_ms: number;
  node_id: string;
}

export interface TestResult {
  success: boolean;
  message: string;
  duration_ms: number;
  executed_at: string;
  criticality?: string;
  details?: any;
}

