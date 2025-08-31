
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
  disabled?: boolean;
  metadata?: any;
}

export interface RichSelectTag {
  text: string;
  textColor: string;
  bgColor: string;
}

export interface ValidationMessage {
  message: string;
  field_id?: string;
  severity: 'Error' | 'Warning' | 'Info';
}
