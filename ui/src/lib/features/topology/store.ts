import { writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import { type Node } from '@xyflow/svelte';
import { EdgeHandle, type TopologyResponse } from './types/base';
import { pushError } from '$lib/shared/stores/feedback';
import { toPng } from 'html-to-image';

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

export async function exportToPNG() {
  const flowElement = document.querySelector('.svelte-flow');
  
  if (!flowElement) {
    pushError('Could not find flow element');
    return;
  }

  flowElement.classList.add('hide-for-export');

  const watermark = document.createElement('div');
  watermark.className = 'export-watermark';
  watermark.textContent = 'created with netvisor.io';
  watermark.style.cssText = `
    position: absolute;
    bottom: 16px;
    right: 16px;
    font-size: 16px;
    color: #9ca3af;
    font-weight: 500;
    z-index: 1000;
    pointer-events: none;
  `;
  flowElement.appendChild(watermark);

  try {
    const dataUrl = await toPng(flowElement as HTMLElement, {
      backgroundColor: '#1f2937',
      pixelRatio: 2,
    });
    
    // Download
    const link = document.createElement('a');
    link.download = `netvisor-topology-${new Date().toISOString().split('T')[0]}.png`;
    link.href = dataUrl;
    link.click();
  } catch (err) {
    pushError(`Failed to export topology: ${err}`);
  } finally {
    flowElement.classList.remove('hide-for-export');
    watermark.remove();
  }
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

function getNodesBounds(nodeList: Node[]) {
  if (nodeList.length === 0) return { x: 0, y: 0, width: 800, height: 600 };
  
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  
  nodeList.forEach(node => {
    minX = Math.min(minX, node.position.x);
    minY = Math.min(minY, node.position.y);
    maxX = Math.max(maxX, node.position.x + (node.width || 0));
    maxY = Math.max(maxY, node.position.y + (node.height || 0));
  });
  
  return {
    x: minX,
    y: minY,
    width: maxX - minX,
    height: maxY - minY
  };
}