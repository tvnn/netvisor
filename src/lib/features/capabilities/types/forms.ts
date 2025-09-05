import type { TypeMetadata } from "$lib/shared/stores/registry";
import type { ConfigField, ValidationMessage } from "../../../shared/components/forms/types";

export interface CapabilityConfigForm {
  capability_info: TypeMetadata;
  capability_fields: ConfigField[];
  // test_sections: TestSection[];
  warnings: ValidationMessage[];
  errors: ValidationMessage[];
  system_assigned: boolean;
}

// export interface TestSection {
//   test_type: string;
//   test_info: TypeMetadata;
//   description: string;
//   assignment_reason: string;
//   enabled_by_default: boolean;
//   test_fields: ConfigField[];
// }