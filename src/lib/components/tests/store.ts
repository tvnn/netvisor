import type { NodeContextForAPI, TestConfigSchema } from './types';

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