import type { Service } from '$lib/features/services/types/base';

export interface NodeBase {
	id: string;
	node_type: string;
	position: { x: number; y: number };
	size: { x: number; y: number };
	header: string | null;
}

type NodeType =
	| {
			node_type: 'InterfaceNode';
			subnet_id: string;
			host_id: string;
			interface_id: string;
			is_infra: boolean;
	  }
	| { node_type: 'SubnetNode'; infra_width: number };

type TopologyNode = NodeBase & NodeType & Record<string, unknown>;

export interface NodeRenderData {
	headerText: string | null;
	footerText: string | null;
	bodyText: string | null;
	showServices: boolean;
	isVirtualized: boolean;
	services: Service[];
}

export interface TopologyEdge extends Record<string, unknown> {
	edge_type: string;
	source: string;
	label: string;
	target: string;
	source_handle: EdgeHandle;
	target_handle: EdgeHandle;
	is_multi_hop: boolean;
}

export interface TopologyResponse {
	edge_property: string;
	edges: Array<[number, number, TopologyEdge]>;
	node_holes: unknown[];
	nodes: TopologyNode[];
}

export enum EdgeHandle {
	Top = 'Top',
	Right = 'Right',
	Bottom = 'Bottom',
	Left = 'Left'
}

export interface TopologyRequestOptions {
	group_docker_bridges_by_host: boolean;
	network_ids: string[];
	show_gateway_as_infra_service: boolean;
	infra_service_categories: string[];
	hide_service_categories: string[];
	edge_type: 'smoothstep' | 'bezier';
}
