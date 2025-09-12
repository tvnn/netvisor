  interface TopologyNodeData {
    color: string;
    icon: string;
    id: string;
    label: string;
    node_type: string;
    parent_id: string | null;
    position: { x: number; y: number };
    size: { x: number; y: number };
  }

  interface TopologyEdgeData {
    edge_type: string;
    source: string;
    target: string;
  }

  interface TopologyResponse {
    edge_property: string;
    edges: Array<[number, number, TopologyEdgeData]>;
    node_holes: any[];
    nodes: TopologyNodeData[];
  }