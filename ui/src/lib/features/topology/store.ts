import { get, writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import { type Node } from '@xyflow/svelte';
import { EdgeHandle, type TopologyResponse, type TopologyRequestOptions } from './types/base';
import { pushError } from '$lib/shared/stores/feedback';
import { toPng } from 'html-to-image';
import { currentNetwork, networks } from '../networks/store';

const OPTIONS_STORAGE_KEY = 'netvisor_topology_options';
const EXPANDED_STORAGE_KEY = 'netvisor_topology_options_expanded_state';

// Default options
const defaultOptions: TopologyRequestOptions = {
	group_docker_bridges_by_host: true,
	show_gateway_as_infra_service: true,
	infra_service_categories: ['DNS', 'ReverseProxy'],
	hide_service_categories: [],
	network_ids: [],
	edge_type: 'smoothstep'
};

// Load options from localStorage or use defaults
function loadOptionsFromStorage(): TopologyRequestOptions {
	if (typeof window === 'undefined') return defaultOptions;

	try {
		const stored = localStorage.getItem(OPTIONS_STORAGE_KEY);
		if (stored) {
			const parsed = JSON.parse(stored);
			// Merge with defaults to ensure all fields exist
			return { ...defaultOptions, ...parsed };
		}
	} catch (error) {
		console.warn('Failed to load topology options from localStorage:', error);
	}
	return defaultOptions;
}

// Save options to localStorage
function saveOptionsToStorage(options: TopologyRequestOptions): void {
	if (typeof window === 'undefined') return;

	try {
		localStorage.setItem(OPTIONS_STORAGE_KEY, JSON.stringify(options));
	} catch (error) {
		console.error('Failed to save topology options to localStorage:', error);
	}
}

// Load options panel expanded state from localStorage or use defaults
function loadExpandedFromStorage(): boolean {
	if (typeof window === 'undefined') return true;

	try {
		const stored = localStorage.getItem(EXPANDED_STORAGE_KEY);
		if (stored) {
			return JSON.parse(stored);
		}
	} catch (error) {
		console.warn('Failed to load topology expanded state from localStorage:', error);
	}
	return false;
}

// Save options to localStorage
function saveExpandedToStorage(expanded: boolean): void {
	if (typeof window === 'undefined') return;

	try {
		localStorage.setItem(EXPANDED_STORAGE_KEY, JSON.stringify(expanded));
	} catch (error) {
		console.error('Failed to save topology expanded state to localStorage:', error);
	}
}

export const topology = writable<TopologyResponse>();
export const topologyOptions = writable<TopologyRequestOptions>(loadOptionsFromStorage());
export const optionsPanelExpanded = writable<boolean>(loadExpandedFromStorage());

// Initialize network_ids with the first network when networks are loaded
let networksInitialized = false;
networks.subscribe(($networks) => {
	if (!networksInitialized && $networks.length > 0) {
		networksInitialized = true;
		topologyOptions.update((opts) => {
			// Only set default if network_ids is empty
			if (opts.network_ids.length === 0 && $networks[0]) {
				opts.network_ids = [$networks[0].id];
			}
			return opts;
		});
	}
});

// Subscribe to options changes and save to localStorage
if (typeof window !== 'undefined') {
	topologyOptions.subscribe((options) => {
		saveOptionsToStorage(options);
	});

	optionsPanelExpanded.subscribe((expanded) => {
		saveExpandedToStorage(expanded);
	});
}

export async function getTopology() {
	const options = get(topologyOptions);
	const network = get(currentNetwork);

	return await api.request<TopologyResponse>('/topology', topology, (topology) => topology, {
		method: 'POST',
		body: JSON.stringify({
			...options,
			network_id: network.id
		})
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

	// Get current transform to extract scale and translation
	const currentTransform = new DOMMatrix(getComputedStyle(viewportElement).transform);
	const scale = currentTransform.a; // scale factor

	// Calculate bounding box of all nodes
	let minX = Infinity,
		minY = Infinity,
		maxX = -Infinity,
		maxY = -Infinity;

	nodeElements.forEach((node) => {
		const rect = node.getBoundingClientRect();
		const viewportRect = viewportElement.getBoundingClientRect();

		// Calculate position in flow coordinates (accounting for scale)
		const x = (rect.left - viewportRect.left) / scale;
		const y = (rect.top - viewportRect.top) / scale;
		const width = rect.width / scale;
		const height = rect.height / scale;

		minX = Math.min(minX, x);
		minY = Math.min(minY, y);
		maxX = Math.max(maxX, x + width);
		maxY = Math.max(maxY, y + height);
	});

	// Add padding around content
	const padding = 50;
	minX -= padding;
	minY -= padding;
	maxX += padding;
	maxY += padding;

	const width = maxX - minX;
	const height = maxY - minY;

	// Store original styles
	const originalTransform = viewportElement.style.transform;
	const originalWidth = flowElement.style.width;
	const originalHeight = flowElement.style.height;
	const originalOverflow = flowElement.style.overflow;

	try {
		// Add visual indicator that export is happening
		flowElement.classList.add('hide-for-export');

		// Reset transform to show all content
		viewportElement.style.transform = `translate(${-minX}px, ${-minY}px) scale(1)`;
		flowElement.style.width = `${width}px`;
		flowElement.style.height = `${height}px`;
		flowElement.style.overflow = 'visible';

		// Add watermark
		const watermark = document.createElement('div');
		watermark.textContent = 'created with netvisor.io';
		watermark.style.cssText = `
      position: absolute;
      bottom: 10px;
      right: 10px;
      color: rgba(255, 255, 255, 0.5);
      font-size: 12px;
      font-family: system-ui;
      pointer-events: none;
      z-index: 9999;
    `;
		flowElement.appendChild(watermark);

		// Wait a moment for styles to apply
		await new Promise((resolve) => setTimeout(resolve, 100));

		// Generate image
		const dataUrl = await toPng(flowElement, {
			width,
			height,
			backgroundColor: '#15131e',
			pixelRatio: 2
		});

		// Download the image
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
		const watermark = flowElement.querySelector('div[style*="position: absolute"]');
		watermark?.remove();
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

	if (nodePosition) {
		const nodeCenterX = nodePosition.x + (node.width || 0) / 2;
		const nodeCenterY = nodePosition.y + (node.height || 0) / 2;

		return Math.sqrt(Math.pow(clickX - nodeCenterX, 2) + Math.pow(clickY - nodeCenterY, 2));
	}

	return Infinity;
}
