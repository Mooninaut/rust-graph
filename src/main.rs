pub mod graph;
pub mod query;
pub mod value;

use graph::Graph;
use value::Value;
use query::Query;

fn main() {

    let mut graph = Graph::new();
    graph.insert("cheese", Value::Float64(64.0));
    graph.insert("danish", Value::from("fancy"));
    graph.insert("banana", 7u64);
    println!("link cheese->danish {:?}", graph.link("cheese", "danish"));
    println!("link cheese->danish {:?}", graph.link("cheese", "danish"));
    println!("link cheese->potato {:?}", graph.link("cheese", "potato"));
    println!("query cheese->danish {:?}", graph.existence_query(Query::LinkFromTo(String::from("cheese"), String::from("danish"))));
    println!("query danish->cheese {:?}", graph.existence_query(Query::LinkFromTo(String::from("danish"), String::from("cheese"))));
    println!("query cheese->potato {:?}", graph.existence_query(Query::LinkFromTo(String::from("cheese"), String::from("potato"))));
    println!("query cheese->? {:?}", graph.existence_query(Query::LinkFrom(String::from("cheese"))));
    println!("query danish->? {:?}", graph.existence_query(Query::LinkFrom(String::from("danish"))));
    println!("query potato->? {:?}", graph.existence_query(Query::LinkFrom(String::from("potato"))));
    println!("query ?->cheese {:?}", graph.existence_query(Query::LinkTo(String::from("cheese"))));
    println!("query ?->danish {:?}", graph.existence_query(Query::LinkTo(String::from("danish"))));
    println!("query ?->potato {:?}", graph.existence_query(Query::LinkTo(String::from("potato"))));
    println!("list query cheese->? {:?}", graph.list_query(Query::LinkFrom(String::from("cheese"))));
    println!("list query danish->? {:?}", graph.list_query(Query::LinkFrom(String::from("danish"))));
    println!("list query potato->? {:?}", graph.list_query(Query::LinkFrom(String::from("potato"))));
    println!("list query ?->cheese {:?}", graph.list_query(Query::LinkTo(String::from("cheese"))));
    println!("list query ?->danish {:?}", graph.list_query(Query::LinkTo(String::from("danish"))));
    println!("list query ?->potato {:?}", graph.list_query(Query::LinkTo(String::from("potato"))));
    println!("unlink cheese->danish {:?}", graph.unlink("cheese", "danish"));
    println!("unlink cheese->danish {:?}", graph.unlink("cheese", "danish"));
    println!("unlink cheese->potato {:?}", graph.unlink("cheese", "potato"));
    println!("query cheese->danish {:?}", graph.existence_query(Query::LinkFromTo(String::from("cheese"), String::from("danish"))));
    println!("query danish->cheese {:?}", graph.existence_query(Query::LinkFromTo(String::from("danish"), String::from("cheese"))));
    println!("query cheese->potato {:?}", graph.existence_query(Query::LinkFromTo(String::from("cheese"), String::from("potato"))));
    println!("query cheese->? {:?}", graph.existence_query(Query::LinkFrom(String::from("cheese"))));
    println!("query danish->? {:?}", graph.existence_query(Query::LinkFrom(String::from("danish"))));
    println!("query potato->? {:?}", graph.existence_query(Query::LinkFrom(String::from("potato"))));
    println!("query ?->cheese {:?}", graph.existence_query(Query::LinkTo(String::from("cheese"))));
    println!("query ?->danish {:?}", graph.existence_query(Query::LinkTo(String::from("danish"))));
    println!("query ?->potato {:?}", graph.existence_query(Query::LinkTo(String::from("potato"))));
}
