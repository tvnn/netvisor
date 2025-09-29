export interface TopologyNodeData {
	id: string;
	node_type: string;
	parent_id: string | null;
	host_id: string | null;
	infra_width: number | null;
	interface_id: string | null;
	position: { x: number; y: number };
	size: { x: number; y: number };
	subnet_label: string | null;
	subnet_type: string | null;
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
