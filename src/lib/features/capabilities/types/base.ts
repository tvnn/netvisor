export interface Capability {
  id: string;
  name: string;                    
  capability_type: string;         
  config: Record<string, any>;     
  tests: CapabilityTest[];         
  removable: boolean;
}

export interface CapabilityTest {
    test: string,
    criticality: string,
    enabled: boolean
}