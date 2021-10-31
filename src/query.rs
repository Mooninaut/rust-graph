use crate::graph::NodeKey;

#[derive(Debug)]
pub enum Query<NodeId: NodeKey> {
    LinkFromTo(NodeId, NodeId),
    Node(NodeId),
    LinkFrom(NodeId),
    LinkTo(NodeId),
}

impl<NodeId: NodeKey> Query<NodeId> {
    pub fn link_from_to(from: &NodeId, to: &NodeId) -> Query<NodeId> {
        Query::LinkFromTo(from.clone(), to.clone())
    }
    pub fn node(node: &NodeId) -> Query<NodeId> {
        Query::Node(node.clone())
    }
    pub fn link_from(from: &NodeId) -> Query<NodeId> {
        Query::LinkFrom(from.clone())
    }
    pub fn link_to(to: &NodeId) -> Query<NodeId> {
        Query::LinkTo(to.clone())
    }
}
