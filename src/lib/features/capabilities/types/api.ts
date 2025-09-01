import type { NodeContext } from "$lib/features/nodes/types/base";
import type { CapabilityConfigForm } from "./forms";

export interface CapabilityFormRequest {
    capability_types?: string[],
    node_context: NodeContext
}