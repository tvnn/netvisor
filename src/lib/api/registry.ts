import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

export interface TypeMetadata {
  id: string;
  display_name: string;
  description: string;
  category: string;
  icon: string;
  color: string;
  metadata: Record<string, any>;
}

export interface TypeRegistry {
  test_types: TypeMetadata[];
  node_types: TypeMetadata[];
  capabilities: TypeMetadata[];
  criticality_levels: TypeMetadata[];
  node_statuses: TypeMetadata[];
  node_targets: TypeMetadata[];
}

// Core registry store
export const typeRegistry = writable<TypeRegistry | null>(null);
export const registryLoading = writable<boolean>(false);
export const registryError = writable<string | null>(null);

// Derived stores for easy reactive access
export const testTypes = derived(typeRegistry, $registry => 
  $registry?.test_types || []
);

export const nodeTypes = derived(typeRegistry, $registry => 
  $registry?.node_types || []
);

export const capabilities = derived(typeRegistry, $registry => 
  $registry?.capabilities || []
);

export const criticalityLevels = derived(typeRegistry, $registry => 
  $registry?.criticality_levels || []
);

export const nodeStatuses = derived(typeRegistry, $registry => 
  $registry?.node_statuses || []
);

export const nodeTargets = derived(typeRegistry, $registry => 
  $registry?.node_targets || []
);

// Derived helper stores
export const getTestType = derived(typeRegistry, $registry => {
  return (id: string) => $registry?.test_types?.find(t => t.id === id) || null;
});

export const getNodeType = derived(typeRegistry, $registry => {
  return (id: string) => $registry?.node_types?.find(t => t.id === id) || null;
});

export const getCapability = derived(typeRegistry, $registry => {
  return (id: string) => $registry?.capabilities?.find(t => t.id === id) || null;
});

export const getCriticality = derived(typeRegistry, $registry => {
  return (id: string) => $registry?.criticality_levels?.find(t => t.id === id) || null;
});

export const getNodeStatus = derived(typeRegistry, $registry => {
  return (id: string) => $registry?.node_statuses?.find(t => t.id === id) || null;
});

export const getNodeTarget = derived(typeRegistry, $registry => {
  return (id: string) => $registry?.node_targets?.find(t => t.id === id) || null;
});


// Display helper stores
export const getTestDisplay = derived(typeRegistry, $registry => {
  return (testType: string) => {
    const test = $registry?.test_types?.find(t => t.id === testType);
    return test?.display_name || testType;
  };
});

export const getNodeTypeDisplay = derived(typeRegistry, $registry => {
  return (nodeType: string) => {
    const type = $registry?.node_types?.find(t => t.id === nodeType);
    return type?.display_name || nodeType;
  };
});

export const getCapabilityDisplay = derived(typeRegistry, $registry => {
  return (capability: string) => {
    const cap = $registry?.capabilities?.find(t => t.id === capability);
    return cap?.display_name || capability;
  };
});

export const getCriticalityDisplay = derived(typeRegistry, $registry => {
  return (criticality: string) => {
    const crit = $registry?.criticality_levels?.find(t => t.id === criticality);
    return crit?.display_name || criticality;
  };
});

export const getNodeStatusDisplay = derived(typeRegistry, $registry => {
  return (status: string) => {
    const stat = $registry?.node_statuses?.find(t => t.id === status);
    return stat?.display_name || status;
  };
});

// Description helper stores

export const getCapabilityDescription = derived(typeRegistry, $registry => {
  return (capability: string) => {
    const cap = $registry?.capabilities?.find(t => t.id === capability);
    return cap?.description || "";
  };
});


// Icon helper stores
export const getTestIcon = derived(typeRegistry, $registry => {
  return (testType: string) => {
    const test = $registry?.test_types?.find(t => t.id === testType);
    return test?.icon || 'help-circle';
  };
});

export const getNodeTypeIcon = derived(typeRegistry, $registry => {
  return (nodeType: string) => {
    const type = $registry?.node_types?.find(t => t.id === nodeType);
    return type?.icon || 'help-circle';
  };
});


// Color helper stores
export const getTestColor = derived(typeRegistry, $registry => {
  return (testType: string) => {
    const test = $registry?.test_types?.find(t => t.id === testType);
    return test?.color || 'text-gray-400';
  };
});

export const getNodeTypeColor = derived(typeRegistry, $registry => {
  return (nodeType: string) => {
    const type = $registry?.node_types?.find(t => t.id === nodeType);
    return type?.color || 'text-gray-400';
  };
});

export const getCapabilityColor = derived(typeRegistry, $registry => {
  return (capability: string) => {
    const cap = $registry?.capabilities?.find(t => t.id === capability);
    return cap?.color || 'text-gray-400';
  };
});

export const getNodeStatusColor = derived(typeRegistry, $registry => {
  return (nodeStatus: string) => {
    const cap = $registry?.node_statuses?.find(t => t.id === nodeStatus);
    return cap?.color || 'text-gray-400';
  };
});

export const getCriticalityColor = derived(typeRegistry, $registry => {
  return (criticality: string) => {
    const cap = $registry?.criticality_levels?.find(t => t.id === criticality);
    return cap?.color || 'text-gray-400';
  };
});


// Category helpers
export const getTestsByCategory = derived(typeRegistry, $registry => {
  return (category: string) => {
    return $registry?.test_types?.filter(t => t.category === category) || [];
  };
});

export const getNodeTypesByCategory = derived(typeRegistry, $registry => {
  return (category: string) => {
    return $registry?.node_types?.filter(t => t.category === category) || [];
  };
});

// Metadata helper stores
export const getNodeTargetMetadata = derived(typeRegistry, $registry => {
  return (target: string) => {
    const targ = $registry?.node_targets?.find(t => t.id === target);
    return targ?.metadata || {};
  };
});


// Load registry function
async function loadRegistry() {
  if (!browser) return;
  
  registryLoading.set(true);
  registryError.set(null);
  
  try {
    const response = await fetch('/api/registry');
    if (!response.ok) {
      throw new Error(`Failed to load registry: ${response.statusText}`);
    }
    
    const data = await response.json();
    if (data.success) {
      typeRegistry.set(data.data);
    } else {
      throw new Error(data.error || 'Failed to load registry');
    }
  } catch (error) {
    console.error('Registry load error:', error);
    registryError.set(error instanceof Error ? error.message : 'Unknown error');
  } finally {
    registryLoading.set(false);
  }
}

// Auto-load on module import
if (browser) {
  loadRegistry();
}

// Refresh function for manual reload
export function refreshRegistry() {
  return loadRegistry();
}