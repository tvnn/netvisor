import type { NodeResult } from "../nodes/types";
import type { TestResult } from "../tests/types";

export interface DiagnosticExecutionApi {
  group_id: string;
  trigger_reason: string;
}

export interface DiagnosticExecution {
  id: string;
  group_id: string;
  status: string;
  trigger_reason: string;
  node_results: NodeResult[];
  generated_remediation_id?: string | null;
  started_at: string;
  completed_at?: string | null;
  created_at: string;
}