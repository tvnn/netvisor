use crate::server::topology::{
    service::{
        context::TopologyContext,
        optimizer::{child_positioner::ChildPositioner, subnet_positioner::SubnetPositioner},
    },
    types::{edges::Edge, nodes::Node},
};

/// High-level optimizer that coordinates crossing reduction and subnet positioning
pub struct TopologyOptimizer<'a> {
    subnet_positioner: SubnetPositioner<'a>,
    child_positioner: ChildPositioner<'a>,
}

impl<'a> TopologyOptimizer<'a> {
    pub fn new(ctx: &'a TopologyContext<'a>) -> Self {
        Self {
            subnet_positioner: SubnetPositioner::new(ctx),
            child_positioner: ChildPositioner::new(ctx),
        }
    }

    /// Main entry point: optimize node positions and fix edge handles
    pub fn optimize_graph(&self, nodes: &mut [Node], edges: &[Edge]) -> Vec<Edge> {
        // Step 1: Order nodes within subnets based on edge flow (topological ordering)
        self.child_positioner.order_nodes_by_flow(nodes, edges);

        // Step 2: Reduce edge crossings and make edge lengths as short as possible
        self.subnet_positioner.optimize_positions(nodes, edges);
        self.child_positioner.optimize_positions(nodes, edges);

        // Step 3: Fix intra-subnet edge handles based on actual positions
        self.child_positioner.fix_intra_subnet_handles(edges, nodes)
    }
}
