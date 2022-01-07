use crate::graph::NodeKey;

#[derive(Debug, Copy, Clone)]
pub enum Query<'a, NodeId: NodeKey> {
    LinkFromTo(&'a NodeId, &'a NodeId),
    Node(&'a NodeId),
    LinkFrom(&'a NodeId),
    LinkTo(&'a NodeId),
}

impl<'a, NodeId: NodeKey> Query<'a, NodeId> {
    pub fn link_from_to(from: &'a NodeId, to: &'a NodeId) -> Query<'a, NodeId> {
        Query::LinkFromTo(from, to)
    }
    pub fn node(node: &NodeId) -> Query<NodeId> {
        Query::Node(node)
    }
    pub fn link_from(from: &NodeId) -> Query<NodeId> {
        Query::LinkFrom(from)
    }
    pub fn link_to(to: &NodeId) -> Query<NodeId> {
        Query::LinkTo(to)
    }
}
