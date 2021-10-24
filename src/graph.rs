use std::collections::{
    HashMap,
    HashSet,
};

use serde::{Deserialize, Serialize};

use crate::value::Value::{self, *};
use crate::query::Query;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinkDirection {
    From,
    To,
}

impl From<u64>            for Value { fn from(unsigned: u64           ) -> Self { Unsigned64(unsigned) } }
impl From<i64>            for Value { fn from(signed  : i64           ) -> Self { Signed64(signed    ) } }
impl From<f64>            for Value { fn from(double  : f64           ) -> Self { Float64(double     ) } }
impl From<Vec<Value>>     for Value { fn from(vector  : Vec<Value>    ) -> Self { List(vector        ) } }

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

pub type NodeId = String;
type LinkIndex = HashMap<(NodeId, LinkDirection), HashSet<NodeId>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Graph {
    nodes: HashMap<NodeId, Value>,
    links: HashSet<(NodeId, NodeId)>,
    link_index: LinkIndex,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
            links: HashSet::new(),
            link_index: LinkIndex::new(),
        }
    }
    pub fn insert<T>(&mut self, name: &str, value: T) -> bool
    where T: Into<Value> {
        if self.nodes.contains_key(name) {
            return false;
        }
        self.nodes.insert(NodeId::from(name), value.into());
        return true;
    }

    fn insert_into_link_index(&mut self, node_id: &NodeId, direction: LinkDirection, other_node_id: &NodeId) {
        let key = (node_id.clone(), direction);

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

    fn remove_from_link_index(&mut self, node_id: &NodeId, direction: LinkDirection, other_node_id: &NodeId) {
        let key = (node_id.clone(), direction);

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
    pub fn link(&mut self, from: &str, to: &str) -> Option<bool> {
        if self.nodes.contains_key(from) && self.nodes.contains_key(to) {
            let from_id = &NodeId::from(from);
            let to_id   = &NodeId::from(to);

            self.insert_into_link_index(from_id, LinkDirection::From, to_id);
            self.insert_into_link_index(to_id  , LinkDirection::To  , from_id);

            Some(self.links.insert((from_id.clone(), to_id.clone())))
        }
        else {
            None
        }
    }

    pub fn unlink(&mut self, from: &str, to: &str) -> Option<bool> {
        if self.nodes.contains_key(from) && self.nodes.contains_key(to) {
            let from_id = &NodeId::from(from);
            let to_id   = &NodeId::from(to);

            if self.links.remove(&(from_id.clone(), to_id.clone())) {

                self.remove_from_link_index(from_id, LinkDirection::From, to_id);
                self.remove_from_link_index(to_id  , LinkDirection::To  , from_id);

                Some(true)
            }
            else {
                Some(false)
            }
        } else {
            None
        }
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        match self.nodes.get(key) {
            Some(result) => Some(result.clone()),
            None => None
        }
    }

    pub fn existence_query(&self, query: Query) -> Option<bool> {
        match query {
            Query::LinkFromTo(n0, n1) => {
                if self.nodes.contains_key(&n0) && self.nodes.contains_key(&n1) {
                    Some(self.links.contains(&(n0, n1)))
                } else {
                    None
                }
            },
            Query::LinkFrom(node) => {
                if self.nodes.contains_key(&node) {
                    Some(self.link_index.contains_key(&(node, LinkDirection::From)))
                } else {
                    None
                }
            },
            Query::LinkTo(node) => {
                if self.nodes.contains_key(&node) {
                    Some(self.link_index.contains_key(&(node, LinkDirection::To)))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    pub fn list_query(&self, query: Query) -> Result<HashSet<NodeId>, String> {
        match query {
            Query::LinkFrom(node) => {
                if self.nodes.contains_key(&node) {
                    if let Some(set) = self.link_index.get(&(node, LinkDirection::From)) {
                        Ok(set.clone())
                    } else {
                        Ok(HashSet::new())
                    }
                } else {
                    Err(format!("Node '{}' does not exist", node))
                }
            },
            Query::LinkTo(node) => {
                if self.nodes.contains_key(&node) {
                    if let Some(set) = self.link_index.get(&(node, LinkDirection::To)) {
                        Ok(set.clone())
                    } else {
                        Ok(HashSet::new())
                    }
                } else {
                    Err(format!("Node '{}' does not exist", node))
                }
            },
            _ => Err(format!("Unknown query type [{:?}]", query)),
        }
    }
}
