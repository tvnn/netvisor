import { writable, derived, type Writable, type Readable } from 'svelte/store';
import { commands } from '../api-client';
import type { Test, NetworkNode, CheckConfig } from '../types';
import { CHECK_TYPES } from './checks';

// Store for all tests
export const tests: Writable<Test[]> = writable([]);

// Test management functions
export const testActions = {
  async add(test: Omit<Test, 'id' | 'createdAt' | 'updatedAt'>): Promise<Test> {
    try {
      const newTest: Test = {
        ...test,
        id: crypto.randomUUID(),
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString()
      };
      
      tests.update(current => [...current, newTest]);
      await commands.saveTest(newTest);
      
      return newTest;
    } catch (error) {
      console.error('Failed to add test:', error);
      throw error;
    }
  },

  async update(id: string, updates: Partial<Test>): Promise<Test> {
    try {
      const updatedTest: Test = {
        ...updates as Test,
        id,
        updatedAt: new Date().toISOString()
      };
      
      tests.update(current =>
        current.map(t => t.id === id ? updatedTest : t)
      );
      
      await commands.updateTest(id, updatedTest);
      
      return updatedTest;
    } catch (error) {
      console.error('Failed to update test:', error);
      throw error;
    }
  },

  async delete(id: string): Promise<void> {
    try {
      tests.update(current => current.filter(t => t.id !== id));
      await commands.deleteTest(id);
    } catch (error) {
      console.error('Failed to delete test:', error);
      throw error;
    }
  },

  async duplicate(id: string): Promise<Test> {
    try {
      // Get current test synchronously
      let currentTests: Test[] = [];
      const unsubscribe = tests.subscribe(value => {
        currentTests = value;
      });
      unsubscribe(); // Immediately unsubscribe after getting the value
      
      const original = currentTests.find(t => t.id === id);
      if (!original) throw new Error('Test not found');
      
      const duplicate: Omit<Test, 'id' | 'createdAt' | 'updatedAt'> = {
        ...original,
        name: `${original.name} (Copy)`
      };
      
      return await this.add(duplicate);
    } catch (error) {
      console.error('Failed to duplicate test:', error);
      throw error;
    }
  }
};

// Validation functions
export function validateTest(test: Test, availableNodes: NetworkNode[] = []): string[] {
  const errors: string[] = [];
  
  if (!test.name?.trim()) {
    errors.push('Name is required');
  }
  
  if (!test.layers || !Array.isArray(test.layers) || test.layers.length === 0) {
    errors.push('At least one layer is required');
  }
  
  // Validate layers
  test.layers?.forEach((layer, layerIndex) => {
    if (!layer.name?.trim()) {
      errors.push(`Layer ${layerIndex + 1}: Name is required`);
    }
    
    if (!layer.checks || !Array.isArray(layer.checks) || layer.checks.length === 0) {
      errors.push(`Layer ${layerIndex + 1}: At least one test is required`);
    }
    
    // Validate checks
    layer.checks?.forEach((check, checkIndex) => {
      if (!check.type || !CHECK_TYPES[check.type]) {
        errors.push(`Layer ${layerIndex + 1}, Test ${checkIndex + 1}: Invalid test type`);
      }
      
      // Validate check configuration based on test type
      const testType = CHECK_TYPES[check.type];
      if (testType) {
        testType.fields.forEach(field => {
          const value = check.config[field as keyof CheckConfig];
          if (field === 'target' || field === 'domain') {
            if (!value || (typeof value === 'string' && !value.trim())) {
              errors.push(`Layer ${layerIndex + 1}, Test ${checkIndex + 1}: ${field} is required`);
            }
          }
        });
      }
    });
  });
  
  return errors;
}

// Helper to create a blank test
export function createBlankTest(): Omit<Test, 'id' | 'createdAt' | 'updatedAt'> {
  return {
    name: '',
    description: '',
    version: '1.0',
    layers: []
  };
}

// Load tests on app start
export async function loadTests(): Promise<void> {
  try {
    const loadedTests = await commands.getTests();
    tests.set(loadedTests);
  } catch (error) {
    console.error('Failed to load tests:', error);
    // Set empty array on error
    tests.set([]);
  }
}