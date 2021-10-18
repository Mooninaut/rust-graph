pub mod graph;
pub mod query;

use graph::{*, Value::*};
use query::Query;

fn main() {

    let mut graph = Graph::new();
    graph.insert("cheese", Float64(64.0));
    graph.insert("danish", Value::from("fancy"));
    graph.insert("banana", 7u64);
    println!("link cheese->danish {:?}", graph.link("cheese", "danish"));
    println!("link cheese->danish {:?}", graph.link("cheese", "danish"));
    println!("link cheese->potato {:?}", graph.link("cheese", "potato"));
    println!("query cheese->danish {:?}", graph.query(Query::TwoNodes(String::from("cheese"), String::from("danish"))));
    println!("query danish->cheese {:?}", graph.query(Query::TwoNodes(String::from("danish"), String::from("cheese"))));
    println!("query cheese->potato {:?}", graph.query(Query::TwoNodes(String::from("cheese"), String::from("potato"))));
    println!("unlink cheese->danish {:?}", graph.unlink("cheese", "danish"));
    println!("unlink cheese->danish {:?}", graph.unlink("cheese", "danish"));
    println!("unlink cheese->potato {:?}", graph.unlink("cheese", "potato"));
}
