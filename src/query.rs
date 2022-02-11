use std::collections::HashSet;

use crate::graph::NodeKey;
use crate::graph::Graph;

#[derive(Debug, Copy, Clone)]
pub enum Query<'a, NodeId: NodeKey> {
    LinkSourceTarget(&'a NodeId, &'a NodeId),
    Node(&'a NodeId),
    LinkSource(&'a NodeId),
    LinkTarget(&'a NodeId),
}

#[derive(Debug, Copy, Clone)]
pub struct GraphQuery<'a, NodeId: NodeKey> {
    graph: &'a Graph<NodeId>,
    query: Query<'a, NodeId>,
}

impl<'a, NodeId: NodeKey> Query<'a, NodeId> {
    pub fn link_source_target(source: &'a NodeId, target: &'a NodeId) -> Query<'a, NodeId> {
        Query::LinkSourceTarget(source, target)
    }
    pub fn node(node: &NodeId) -> Query<NodeId> {
        Query::Node(node)
    }
    pub fn link_source(source: &NodeId) -> Query<NodeId> {
        Query::LinkSource(source)
    }
    pub fn link_target(target: &NodeId) -> Query<NodeId> {
        Query::LinkTarget(target)
    }
}

impl<'a, NodeId: NodeKey> GraphQuery<'a, NodeId> {
    pub fn new(graph: &'a Graph<NodeId>, query: Query<'a, NodeId>) -> GraphQuery<'a, NodeId> {
        GraphQuery { graph, query }
    }

    pub fn source(&self, source: &'a NodeId) -> GraphQuery<'a, NodeId> {
        match self.query {
            Query::LinkTarget(target) => GraphQuery { graph: self.graph, query: Query::link_source_target(source, target) },
            _ => panic!("can't do LinkSource when self isn't LinkTarget") // todo
        }
    }

    pub fn target(&self, target: &'a NodeId) -> GraphQuery<'a, NodeId> {
        match self.query {
            Query::LinkSource(source) => GraphQuery { graph: self.graph, query: Query::link_source_target(source, target) },
            _ => panic!("can't do LinkTarget when self isn't LinkSource") // todo
        }
    }

    pub fn exists(&self) -> Option<bool> {
        self.graph.existence_query(&self.query)
    }

    pub fn as_set(&self) -> Result<HashSet<NodeId>, String> {
        self.graph.set_query(&self.query)
    }
}
