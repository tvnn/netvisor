import { writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import { type Node } from '@xyflow/svelte';
import { EdgeHandle, type TopologyResponse } from './types/base';
import { pushError } from '$lib/shared/stores/feedback';
import { toPng } from 'html-to-image';

export const topology = writable<TopologyResponse>();

export async function getTopology() {
	return await api.request<TopologyResponse>('/topology', topology, (topology) => topology, {
		method: 'GET'
	});
}

export async function exportToPNG() {
	const flowElement = document.querySelector('.svelte-flow') as HTMLElement;
	const viewportElement = document.querySelector('.svelte-flow__viewport') as HTMLElement;

	if (!flowElement || !viewportElement) {
		pushError('Could not find flow element');
		return;
	}

	// Get all node elements to calculate bounds
	const nodeElements = flowElement.querySelectorAll('.svelte-flow__node');
	if (nodeElements.length === 0) {
		pushError('No nodes to export');
		return;
	}

	// Calculate bounding box of all nodes in viewport coordinates
	let minX = Infinity,
		minY = Infinity,
		maxX = -Infinity,
		maxY = -Infinity;

	nodeElements.forEach((node) => {
		const rect = node.getBoundingClientRect();
		const viewportRect = viewportElement.getBoundingClientRect();

		// Calculate position relative to viewport
		const x = rect.left - viewportRect.left;
		const y = rect.top - viewportRect.top;

		minX = Math.min(minX, x);
		minY = Math.min(minY, y);
		maxX = Math.max(maxX, x + rect.width);
		maxY = Math.max(maxY, y + rect.height);
	});

	// Add padding
	const padding = 40;
	const contentWidth = maxX - minX + padding * 2;
	const contentHeight = maxY - minY + padding * 2;

	// Store original styles
	const originalTransform = viewportElement.style.transform;
	const originalWidth = flowElement.style.width;
	const originalHeight = flowElement.style.height;
	const originalOverflow = flowElement.style.overflow;

	// Adjust the flow element to fit content
	flowElement.style.width = `${contentWidth}px`;
	flowElement.style.height = `${contentHeight}px`;
	flowElement.style.overflow = 'hidden';

	// Shift viewport to show content from top-left with padding
	const currentTransform = new DOMMatrix(getComputedStyle(viewportElement).transform);
	viewportElement.style.transform = `translate(${-minX + padding + currentTransform.e}px, ${-minY + padding + currentTransform.f}px) scale(${currentTransform.a})`;

	flowElement.classList.add('hide-for-export');

	const watermark = document.createElement('div');
	watermark.className = 'export-watermark';
	watermark.textContent = 'created with netvisor.io';
	watermark.style.cssText = `
		position: absolute;
		bottom: 16px;
		right: 16px;
		font-size: 14px;
		color: #9ca3af;
		font-weight: 500;
		z-index: 1000;
		pointer-events: none;
	`;
	flowElement.appendChild(watermark);

	try {
		const dataUrl = await toPng(flowElement, {
			backgroundColor: '#1f2937',
			pixelRatio: 2
		});

		const link = document.createElement('a');
		link.download = `netvisor-topology-${new Date().toISOString().split('T')[0]}.png`;
		link.href = dataUrl;
		link.click();
	} catch (err) {
		pushError(`Failed to export topology: ${err}`);
	} finally {
		// Restore original styles
		viewportElement.style.transform = originalTransform;
		flowElement.style.width = originalWidth;
		flowElement.style.height = originalHeight;
		flowElement.style.overflow = originalOverflow;
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

function getAbsoluteNodePosition(nodeId: string): { x: number; y: number } | null {
	// Find the DOM element for the node
	const nodeElement = document.querySelector(`[data-id="${nodeId}"]`);
	if (!nodeElement) return null;

	const rect = nodeElement.getBoundingClientRect();
	return {
		x: rect.left + rect.width / 2, // Center X
		y: rect.top + rect.height / 2 // Center Y
	};
}

// Calculate distance between click point and node centers
export function getDistanceToNode(clickX: number, clickY: number, node: Node): number {
	const nodePosition = getAbsoluteNodePosition(node.id);

	console.log(nodePosition);

	if (nodePosition) {
		const nodeCenterX = nodePosition.x + (node.width || 0) / 2;
		const nodeCenterY = nodePosition.y + (node.height || 0) / 2;

		return Math.sqrt(Math.pow(clickX - nodeCenterX, 2) + Math.pow(clickY - nodeCenterY, 2));
	}

	return Infinity;
}
