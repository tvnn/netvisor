export interface TestResult {
  success: boolean;
  message: string;
  duration_ms: number;
  executed_at: string;
  criticality?: string;
  details?: any;
}

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
  target: any;
}