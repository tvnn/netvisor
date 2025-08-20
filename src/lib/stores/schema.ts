import { writable } from 'svelte/store';

export interface TestConfigSchema {
  test_info: {
    id: string;
    display_name: string;
    description: string;
    category: string;
    icon: string;
    color: string;
  };
  contextual_description: string;
  compatibility: 'Compatible' | 'Incompatible' | 'Conditional';
  requirements_met: boolean;
  compatibility_reason?: string;
  fields: ConfigField[];
  warnings: ValidationMessage[];
  errors: ValidationMessage[];
}

export interface ConfigField {
  id: string;
  label: string;
  field_type: {
    base_type: string;
    constraints: Record<string, any>;
    options?: SelectOption[];
  };
  required: boolean;
  default_value?: any;
  help_text?: string;
  placeholder?: string;
  advanced: boolean;
}

export interface SelectOption {
  value: string;
  label: string;
  description?: string;
  disabled: boolean;
}

export interface ValidationMessage {
  message: string;
  field_id?: string;
  severity: 'Error' | 'Warning' | 'Info';
}

export interface NodeContextForAPI {
  node_id?: string;
  node_type: string;
  capabilities: string[];
  target: any; // Just pass through the existing NodeTarget
  assigned_tests: string[];
}

export async function fetchTestSchema(testType: string, nodeContext: NodeContextForAPI): Promise<TestConfigSchema> {
  const response = await fetch('/api/tests/schema', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      test_type: testType,
      node_context: nodeContext,
    }),
  });
  
  if (!response.ok) {
    throw new Error(`Failed to fetch schema: ${response.statusText}`);
  }
  
  const data = await response.json();
  if (!data.success) {
    throw new Error(data.error || 'Failed to fetch schema');
  }
  
  return data.data;
}