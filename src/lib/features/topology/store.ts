import { writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import { type Node } from '@xyflow/svelte';
import { EdgeHandle, type TopologyResponse } from './types/base';

export const topology = writable<TopologyResponse>();

export async function getTopology() {
  return await api.request<TopologyResponse>(
    '/topology',
    topology,
    (topology) => topology,
    { method: 'GET' },
    true
  )
}

// Cycle through anchor positions in logical order
export function getNextHandle(currentHandle: EdgeHandle): EdgeHandle {
  const cycle = [EdgeHandle.Top, EdgeHandle.Right, EdgeHandle.Bottom, EdgeHandle.Left];
  const currentIndex = cycle.indexOf(currentHandle);
  const nextIndex = (currentIndex + 1) % cycle.length;
  return cycle[nextIndex];
}

function getAbsoluteNodePosition(nodeId: string): { x: number, y: number } | null {
  // Find the DOM element for the node
  const nodeElement = document.querySelector(`[data-id="${nodeId}"]`);
  if (!nodeElement) return null;
  
  const rect = nodeElement.getBoundingClientRect();
  return {
    x: rect.left + rect.width / 2,  // Center X
    y: rect.top + rect.height / 2   // Center Y
  };
}

// Calculate distance between click point and node centers
export function getDistanceToNode(clickX: number, clickY: number, node: Node): number {

  let nodePosition = getAbsoluteNodePosition(node.id)

  console.log(nodePosition)

  if (nodePosition) {
    const nodeCenterX = nodePosition.x + (node.width || 0) / 2;
    const nodeCenterY = nodePosition.y + (node.height || 0) / 2;
    
    return Math.sqrt(
      Math.pow(clickX - nodeCenterX, 2) + Math.pow(clickY - nodeCenterY, 2)
    );
  }

  return Infinity
}