use std::collections::{
    HashMap,
    HashSet,
};

use std::hash::Hash;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::value::Value::{self, *};
use crate::query::Query;
use crate::query::GraphQuery;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Endpoint {
    Source,
    Target,
}

impl From<u64>        for Value { fn from(unsigned: u64       ) -> Self { Unsigned64(unsigned) } }
impl From<i64>        for Value { fn from(signed  : i64       ) -> Self { Signed64(  signed  ) } }
impl From<f64>        for Value { fn from(double  : f64       ) -> Self { Float64(   double  ) } }
impl From<Vec<Value>> for Value { fn from(vector  : Vec<Value>) -> Self { List(      vector  ) } }

impl From<&str> for Value {
    fn from(slice: &str) -> Self {
        Text(String::from(slice))
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Text(string)
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(hash_map: HashMap<String, Value>) -> Self {
        Map(hash_map)
    }
}

pub trait NodeKey : Eq + Hash + Clone + Debug {}

impl<T> NodeKey for T where T: Eq + Hash + Clone + Debug {}

pub type LinkIndex<T> = HashMap<(Endpoint, T), HashSet<T>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Graph<NodeId: NodeKey> {
    nodes: HashMap<NodeId, Value>,
    links: HashSet<(NodeId, NodeId)>,
    link_index: LinkIndex<NodeId>,
}

impl<NodeId: NodeKey> Graph<NodeId> {
    pub fn new() -> Graph<NodeId> {
        Graph {
            nodes: HashMap::new(),
            links: HashSet::new(),
            link_index: LinkIndex::new(),
        }
    }
    pub fn insert<T>(&mut self, name: NodeId, value: T) -> bool
            where T: Into<Value> {
        let name: NodeId = name.into();

        if self.nodes.contains_key(&name) {
            return false;
        }

        self.nodes.insert(NodeId::from(name), value.into());

        return true;
    }

    pub fn delete(&mut self, node_id: &NodeId) -> bool {

        if !self.nodes.contains_key(node_id) {
            return false;
        }

        if let Ok(nodes) = self.set_query(&Query::link_source(node_id)) {
            for node in nodes {
                //println!("{:?} from {:?}", node, &node_id);
                self.remove_from_link_index(&node, Endpoint::Target, node_id);
                self.link_index.remove(&(Endpoint::Source, node_id.clone()));
                self.links.remove(&(node_id.clone(), node));
            }
        }
        if let Ok(nodes) = self.set_query(&Query::link_target(node_id)) {
            for node in nodes {
                //println!("{:?} to {:?}", node, &node_id);
                self.remove_from_link_index(&node, Endpoint::Source, node_id);
                self.link_index.remove(&(Endpoint::Target, node_id.clone()));
                self.links.remove(&(node, node_id.clone()));
            }
        }
        self.nodes.remove(node_id);
        return true;
    }

    fn insert_into_link_index(&mut self, node_id: &NodeId, direction: Endpoint, other_node_id: &NodeId) {
        let key = (direction, node_id.clone());

        if self.link_index.contains_key(&key) {
            if let Some(set) = self.link_index.get_mut(&key) {
                set.insert(other_node_id.clone());
            }
            else {
                eprintln!("[WARN] rust-graph::graph key '{:?}' is present but value is not!", key);
            }
        } else {
            let mut set = HashSet::with_capacity(1);

            set.insert(other_node_id.clone());

            if let Some(_old_set) = self.link_index.insert(key.clone(), set) {
                eprintln!("[WARN] rust-graph::graph inserted set for '{:?}' but one was already present!", key)
            }
        }
    }

    fn remove_from_link_index(&mut self, node_id: &NodeId, direction: Endpoint, other_node_id: &NodeId) {
        let key = (direction, node_id.clone());

        if self.link_index.contains_key(&key) {
            if let Some(set) = self.link_index.get_mut(&key) {
                set.remove(other_node_id);
            }
            else {
                eprintln!("[WARN] rust-graph::graph key '{:?}' is present but value is not!", key);
            }
        }
    }

    // Returns true if the value was added, false if it was not because it was already present,
    // and None if the strings are not valid node IDs
    // todo: enforce constraints
    pub fn link(&mut self, source: &NodeId, target: &NodeId) -> Option<bool> {

        if self.nodes.contains_key(source) && self.nodes.contains_key(target) {

            self.insert_into_link_index(source, Endpoint::Source, target);
            self.insert_into_link_index(target, Endpoint::Target, source);

            Some(self.links.insert((source.clone(), target.clone())))
        }
        else {
            None
        }
    }

    pub fn unlink(&mut self, source: &NodeId, target: &NodeId) -> Option<bool> {

        if self.nodes.contains_key(source) && self.nodes.contains_key(target) {

            if self.links.remove(&(source.clone(), target.clone())) {

                self.remove_from_link_index(source, Endpoint::Source, target);
                self.remove_from_link_index(target, Endpoint::Target, source);

                Some(true)
            }
            else {
                Some(false)
            }
        } else {
            None
        }
    }

    pub fn get(&self, key: &NodeId) -> Option<Value> {
        match self.nodes.get(key) {
            Some(result) => Some(result.clone()),
            None => None
        }
    }

    pub fn query_link_source<'a>(&'a self, source_id: &'a NodeId) -> GraphQuery<'a, NodeId> {
        GraphQuery::new(self, Query::link_source(source_id))
    }

    pub fn query_link_target<'a>(&'a self, to_id: &'a NodeId) -> GraphQuery<'a, NodeId> {
        GraphQuery::new(self, Query::link_target(to_id))
    }

    pub fn query_node<'a>(&'a self, node_id: &'a NodeId) -> GraphQuery<'a, NodeId> {
        GraphQuery::new(self, Query::node(node_id))
    }

    pub fn existence_query(&self, query: &Query<NodeId>) -> Option<bool> {
        match query {
            &Query::LinkSourceTarget(n0, n1) => {
                if self.nodes.contains_key(&n0) && self.nodes.contains_key(&n1) {
                    Some(self.links.contains(&(n0.clone(), n1.clone())))
                } else {
                    None
                }
            },
            &Query::LinkSource(node) => {
                if self.nodes.contains_key(&node) {
                    Some(self.link_index.contains_key(&(Endpoint::Source, node.clone())))
                } else {
                    None
                }
            },
            &Query::LinkTarget(node) => {
                if self.nodes.contains_key(&node) {
                    Some(self.link_index.contains_key(&(Endpoint::Target, node.clone())))
                } else {
                    None
                }
            },
            &Query::Node(node) => Some(self.nodes.contains_key(&node)),
            _ => None,
        }
    }

    pub fn set_query(&self, query: &Query<NodeId>) -> Result<HashSet<NodeId>, String> {
        match query {
            &Query::LinkSource(node) => {
                if self.nodes.contains_key(node) {
                    if let Some(set) = self.link_index.get(&(Endpoint::Source, node.clone())) {
                        Ok(set.clone())
                    } else {
                        Ok(HashSet::new())
                    }
                } else {
                    Err(format!("Node '{:?}' does not exist", node))
                }
            },
            &Query::LinkTarget(node) => {
                if self.nodes.contains_key(node) {
                    if let Some(set) = self.link_index.get(&(Endpoint::Target, node.clone())) {
                        Ok(set.clone())
                    } else {
                        Ok(HashSet::new())
                    }
                } else {
                    Err(format!("Node '{:?}' does not exist", node))
                }
            },
            _ => Err(format!("Unknown query type [{:?}]", query)),
        }
    }
}
