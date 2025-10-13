export interface TopologyNodeData {
	id: string;
	node_type: string;
	subnet_id: string | null;
	host_id: string | null;
	infra_width: number | null;
	interface_id: string | null;
	position: { x: number; y: number };
	size: { x: number; y: number };
	subnet_type: string | null;
	label_override: string | null;
}

export interface TopologyEdgeData {
	edge_type: string;
	source: string;
	label: string;
	target: string;
	source_handle: EdgeHandle;
	target_handle: EdgeHandle;
}

export interface TopologyResponse {
	edge_property: string;
	edges: Array<[number, number, TopologyEdgeData]>;
	node_holes: unknown[];
	nodes: TopologyNodeData[];
}

export enum EdgeHandle {
	Top = 'Top',
	Right = 'Right',
	Bottom = 'Bottom',
	Left = 'Left'
}

export interface CustomEdgeData extends Record<string, unknown> {
	edgeType: string;
	label: string | null;
	sourceHandle: EdgeHandle;
	targetHandle: EdgeHandle;
}

export interface CustomNodeData extends Record<string, unknown> {
	id: string;
	host_id: string | null;
	interface_id: string | null;
	infra_width: number | null;
	nodeType: string;
	parentId: string | null;
	width: number;
	height: number;
	subnet_type: string | null;
	label_override: string | null;
}

export interface TopologyRequestOptions {
	group_docker_bridges_by_host: boolean;
}
