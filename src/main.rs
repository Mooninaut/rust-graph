use std::collections::{
    HashMap,
    HashSet,
};

use std::hash::{
    Hasher,
    Hash,
};

use serde::{Deserialize, Serialize};

use Value::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    Text(String),
    Unsigned64(u64),
    Signed64(i64),
    Float64(f64),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
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

pub struct Node {
    name: String,
    links: HashSet<String>,
    value: Value,
}

impl <'a> PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Node { }

impl Hash for Node {
    fn hash<H: Hasher> (&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Node {
    pub fn new<T>(name: &str, value: T) -> Node
    where T: Into<Value> {
        Node { name: String::from(name), links: HashSet::new(), value: value.into() }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn links(&self) -> &HashSet<String> {
        &self.links
    }
    pub fn value(&self) -> &Value {
        &self.value
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Graph {
    nodes: HashMap<String, Value>,
    links: HashSet<(String, String)>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph{ nodes: HashMap::new(), links: HashSet::new() }
    }
    pub fn insert<T>(&mut self, name: &str, value: T) -> bool
    where T: Into<Value> {
        if self.nodes.contains_key(name) {
            return false;
        }
        self.nodes.insert(String::from(name), value.into());
        return true;
    }
    // Returns true if the value was added, false if it was not because it was already present,
    // and Err if the strings are not valid node IDs
    // todo: enforce constraints
    pub fn link(&mut self, from: &str, to: &str) -> Option<bool> {
        if self.nodes.contains_key(from) && self.nodes.contains_key(to) {
            Some(self.links.insert((String::from(from), String::from(to))))
        } else {
            None
        }
    }
    pub fn unlink(&mut self, from: &str, to: &str) -> Option<bool> {
        if self.nodes.contains_key(from) && self.nodes.contains_key(to) {
            Some(self.links.remove(&(String::from(from), String::from(to))))
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
}

fn main() {

    let mut graph = Graph::new();
    graph.insert("cheese", Float64(64.0));
    graph.insert("danish", Value::from("fancy"));
    graph.insert("banana", 7u64);
    println!("{:?}", graph.link("cheese", "danish"));
    println!("{:?}", graph.link("cheese", "danish"));
    println!("{:?}", graph.link("cheese", "potato"));
    println!("{:?}", graph.unlink("cheese", "danish"));
    println!("{:?}", graph.unlink("cheese", "danish"));
    println!("{:?}", graph.unlink("cheese", "potato"));
}
