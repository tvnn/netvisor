import { writable, derived, type Writable, type Readable } from 'svelte/store';
import { commands } from '../api-client';
import type { NetworkNode, ValidationResult, Test } from '../types';

// Store for all network nodes
export const nodes: Writable<NetworkNode[]> = writable([]);

// Node management functions
export const nodeActions = {
  async add(node: Omit<NetworkNode, 'id' | 'createdAt' | 'updatedAt'>): Promise<NetworkNode> {
    try {
      // Add ID and timestamps
      const newNode: NetworkNode = {
        ...node,
        id: crypto.randomUUID(),
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString()
      };
      
      // Update store
      nodes.update(current => [...current, newNode]);
      
      // Persist to Tauri
      await commands.saveNode(newNode);
      
      return newNode;
    } catch (error) {
      console.error('Failed to add node:', error);
      throw error;
    }
  },

  async update(id: string, updates: Partial<NetworkNode>): Promise<NetworkNode> {
    try {
      const updatedNode: NetworkNode = {
        ...updates as NetworkNode,
        id,
        updatedAt: new Date().toISOString()
      };
      
      nodes.update(current => 
        current.map(node => node.id === id ? updatedNode : node)
      );
      
      await commands.updateNode(id, updatedNode);
      
      return updatedNode;
    } catch (error) {
      console.error('Failed to update node:', error);
      throw error;
    }
  },

  async delete(id: string): Promise<void> {
    try {
      nodes.update(current => current.filter(node => node.id !== id));
      await commands.deleteNode(id);
    } catch (error) {
      console.error('Failed to delete node:', error);
      throw error;
    }
  },

  async duplicate(id: string): Promise<NetworkNode> {
    try {
      // Get current nodes synchronously
      let currentNodes: NetworkNode[] = [];
      const unsubscribe = nodes.subscribe(value => {
        currentNodes = value;
      });
      unsubscribe(); // Immediately unsubscribe after getting the value
      
      const original = currentNodes.find(node => node.id === id);
      if (!original) throw new Error('Node not found');
      
      const duplicate: Omit<NetworkNode, 'id' | 'createdAt' | 'updatedAt'> = {
        ...original,
        name: `${original.name} (Copy)`
      };
      
      return await this.add(duplicate);
    } catch (error) {
      console.error('Failed to duplicate node:', error);
      throw error;
    }
  }
};

export function validateNode(node: Partial<NetworkNode>): string[] {
  const errors: string[] = [];
  
  if (!node.name?.trim()) {
    errors.push('Name is required');
  }
  
  if (!node.domain?.trim() && !node.ip?.trim()) {
    errors.push('Either domain or IP address is required');
  }
  
  if (node.ip && !isValidIP(node.ip)) {
    errors.push('Invalid IP address format');
  }
  
  if (node.defaultPort && (!Number.isInteger(Number(node.defaultPort)) || Number(node.defaultPort) < 1 || Number(node.defaultPort) > 65535)) {
    errors.push('Port must be between 1 and 65535');
  }
  
  return errors;
}

function isValidIP(ip: string): boolean {
  const ipRegex = /^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;
  return ipRegex.test(ip);
}

// Helper to get node by ID
export async function getNodeById(nodeId: string): Promise<NetworkNode | undefined> {
  return new Promise(resolve => {
    const unsubscribe = nodes.subscribe(current => {
      unsubscribe();
      resolve(current.find(node => node.id === nodeId));
    });
  });
}

// Helper to get nodes referenced in a test
export function getNodesReferencedInTest(test: Test): string[] {
  const referencedNodeIds = new Set<string>();
  
  // Look for node references in test configurations
  function findNodeReferences(obj: any): void {
    if (typeof obj === 'string' && obj.startsWith('{{') && obj.endsWith('}}')) {
      // Extract node reference like {{node-id.field}} or {{node-id}}
      const match = obj.match(/^\{\{([^.}]+)(?:\.[^}]+)?\}\}$/);
      if (match) {
        referencedNodeIds.add(match[1]);
      }
    } else if (typeof obj === 'object' && obj !== null) {
      Object.values(obj).forEach(findNodeReferences);
    }
  }
  
  // Search through all test configurations
  test.layers?.forEach(layer => {
    layer.checks?.forEach(test => {
      findNodeReferences(test.config);
    });
  });
  
  return Array.from(referencedNodeIds);
}

// Computed store for getting referenced nodes for a test
export function getReferencedNodes(test: Test): Readable<NetworkNode[]> {
  return derived(nodes, ($nodes) => {
    const referencedIds = getNodesReferencedInTest(test);
    return $nodes.filter(node => referencedIds.includes(node.id));
  });
}

// Helper to create a blank node
export function createBlankNode(): Omit<NetworkNode, 'id' | 'createdAt' | 'updatedAt'> {
  return {
    name: '',
    domain: '',
    ip: '',
    defaultPort: undefined,
    path: '',
    description: ''
  };
}

// Load nodes on app start
export async function loadNodes(): Promise<void> {
  try {
    const loadedNodes = await commands.getNodes();
    nodes.set(loadedNodes);
  } catch (error) {
    console.error('Failed to load nodes:', error);
    // Set empty array on error
    nodes.set([]);
  }
}