use std::hash::{
    Hasher,
    Hash,
};

struct Node {
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
