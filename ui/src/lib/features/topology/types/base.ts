export interface NodeBase {
	id: string;
	node_type: string;
	position: { x: number; y: number };
	size: { x: number; y: number };
	header: string | null;
}

type NodeType =
	| {
			node_type: 'HostNode';
			subnet_id: string;
			host_id: string;
			interface_id: string;
			is_infra: boolean;
	  }
	| { node_type: 'SubnetNode'; infra_width: number };

type TopologyNode = NodeBase & NodeType & Record<string, unknown>;

export interface TopologyEdge extends Record<string, unknown> {
	edge_type: string;
	source: string;
	label: string;
	target: string;
	source_handle: EdgeHandle;
	target_handle: EdgeHandle;
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
}
