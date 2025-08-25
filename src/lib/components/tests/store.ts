import type { NodeContextForAPI, TestConfigSchema } from './types';

export async function fetchTestSchemas(
  nodeContext: NodeContextForAPI,
  testTypes?: string[]  // Optional - if not provided, gets all schemas
): Promise<Record<string, TestConfigSchema>> {
  const response = await fetch('/api/tests/schemas', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      test_types: testTypes,  // Will be null/undefined if not provided
      node_context: nodeContext,
    }),
  });
  
  if (!response.ok) {
    throw new Error(`Failed to fetch schemas: ${response.statusText}`);
  }
  
  const data = await response.json();
  if (!data.success) {
    throw new Error(data.error || 'Failed to fetch schemas');
  }
  
  return data.data.schemas;
}