import type { NodeStatus } from "./nodes";
import type { TestConfiguration, TestResult, TestType } from "./tests";


export interface DiagnosticExecution {
  id: string;
  group_id: string;
  group_name: string;
  trigger_reason: string;
  node_results: NodeTestResults[];
  overall_status: DiagnosticStatus;
  generated_remediation_id?: string;
  created_at: string;
}

export interface NodeTestResults {
  node_id: string;
  test_results: TestResult[];
  node_status: NodeStatus;
}

export type DiagnosticStatus = 'Success' | 'PartialFail' | 'Failed' | 'InProgress';export interface Remediation {
  id: string;
  title: string;
  description: string;
  steps: RemediationStep[];
  generated_from_diagnostic?: string;
  created_at: string;
}

export type RemediationStep = {
  UserAction: {
    description: string;
    instructions: string;
    verification_prompt: string;
  };
} |
{
  ServerAction: {
    description: string;
    target_node_id: string;
    test_type: TestType;
    test_config: TestConfiguration;
  };
};

