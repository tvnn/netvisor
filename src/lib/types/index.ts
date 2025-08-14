import type { Node } from "./nodes";
import type { TestType, TestResult } from "./tests";

// Components

export interface CardAction {
  label: string;
  icon: any;
  color?: string;
  hoverColor?: string;
  bgHover?: string;
  onClick: () => void;
  disabled?: boolean;
}

export interface CardSection {
  label: string;
  value: string;
}

export interface CardList {
  label: string;
  items: CardListItem[];
  emptyText?: string;
  renderItem?: (item: CardListItem) => string;
  itemActions?: (item: CardListItem) => CardAction[];
}

export interface CardListItem {
  id: string;
  label: string;
  metadata?: Record<string, any>;
  color?: string;
  bgColor?: string;
  disabled?: boolean;
}

// API Response types
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}

export interface TestExecutionResponse {
  result: TestResult;
}

export interface NodeTestExecutionResponse {
  node_id: string;
  node_name: string;
  results: TestResult[];
  previous_status: string;
  new_status: string;
  executed_at: string;
}

export interface TestTypeInfo {
  test_type: TestType;
  display_name: string;
  description: string;
  required_capabilities: string[];
  required_node_types: string[];
  forbidden_node_types: string[];
}

export interface TestAssignmentApiResponse {
  node: Node;
  warning?: string;
}

export interface CompatibilityResponse {
  node_id: string;
  node_name: string;
  node_type?: string;
  recommended_tests: TestTypeCompatibilityInfo[];
  other_tests: TestTypeCompatibilityInfo[];
}

export interface TestTypeCompatibilityInfo {
  test_type: TestType;
  display_name: string;
  description: string;
  is_assigned: boolean;
  warning?: string;
  is_recommended: boolean;
}