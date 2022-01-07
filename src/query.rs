use std::collections::HashSet;

use crate::graph::NodeKey;
use crate::graph::Graph;

#[derive(Debug, Copy, Clone)]
pub enum Query<'a, NodeId: NodeKey> {
    LinkFromTo(&'a NodeId, &'a NodeId),
    Node(&'a NodeId),
    LinkFrom(&'a NodeId),
    LinkTo(&'a NodeId),
}

#[derive(Debug, Copy, Clone)]
pub struct GraphQuery<'a, NodeId: NodeKey> {
    graph: &'a Graph<NodeId>,
    query: Query<'a, NodeId>,
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

impl<'a, NodeId: NodeKey> GraphQuery<'a, NodeId> {
    pub fn new(graph: &'a Graph<NodeId>, query: Query<'a, NodeId>) -> GraphQuery<'a, NodeId> {
        GraphQuery { graph, query }
    }

    pub fn from(&self, from: &'a NodeId) -> GraphQuery<'a, NodeId> {
        match self.query {
            Query::LinkTo(to) => GraphQuery { graph: self.graph, query: Query::link_from_to(from, to) },
            _ => panic!("can't do LinkFrom when self isn't LinkTo") // todo
        }
    }

    pub fn to(&self, to: &'a NodeId) -> GraphQuery<'a, NodeId> {
        match self.query {
            Query::LinkFrom(from) => GraphQuery { graph: self.graph, query: Query::link_from_to(from, to) },
            _ => panic!("can't do LinkTo when self isn't LinkFrom") // todo
        }
    }

    pub fn exists(&self) -> Option<bool> {
        self.graph.existence_query(&self.query)
    }

    pub fn as_set(&self) -> Result<HashSet<NodeId>, String> {
        self.graph.set_query(&self.query)
    }
}
