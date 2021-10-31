pub mod graph;
pub mod query;
pub mod value;

use graph::Graph;
use value::Value;
use query::Query;

fn main() {
    let cheese = String::from("cheese");
    let danish = String::from("danish");
    let banana = String::from("banana");
    let potato = String::from("potato");

    let mut graph: Graph<String> = Graph::new();
    graph.insert(&cheese, Value::Float64(64.0/0.99));
    graph.insert(&danish, Value::from("fancy"));
    graph.insert(&banana, 7u64);
    println!("link cheese->danish {:?}", graph.link(&cheese, &danish));
    println!("link cheese->danish {:?}", graph.link(&cheese, &danish));
    println!("link cheese->potato {:?}", graph.link(&cheese, &potato));
    println!("serialized: {}", ron::ser::to_string_pretty(&graph, ron::ser::PrettyConfig::new()).unwrap());
    println!("query cheese->danish {:?}", graph.existence_query(Query::link_from_to(&cheese, &danish)));
    println!("query danish->cheese {:?}", graph.existence_query(Query::link_from_to(&danish, &cheese)));
    println!("query cheese->potato {:?}", graph.existence_query(Query::link_from_to(&cheese, &potato)));
    println!("query cheese->? {:?}", graph.existence_query(Query::link_from(&cheese)));
    println!("query danish->? {:?}", graph.existence_query(Query::link_from(&danish)));
    println!("query potato->? {:?}", graph.existence_query(Query::link_from(&potato)));
    println!("query ?->cheese {:?}", graph.existence_query(Query::link_to(&cheese)));
    println!("query ?->danish {:?}", graph.existence_query(Query::link_to(&danish)));
    println!("query ?->potato {:?}", graph.existence_query(Query::link_to(&potato)));
    println!("list query cheese->? {:?}", graph.list_query(Query::link_from(&cheese)));
    println!("list query danish->? {:?}", graph.list_query(Query::link_from(&danish)));
    println!("list query potato->? {:?}", graph.list_query(Query::link_from(&potato)));
    println!("list query ?->cheese {:?}", graph.list_query(Query::link_to(&cheese)));
    println!("list query ?->danish {:?}", graph.list_query(Query::link_to(&danish)));
    println!("list query ?->potato {:?}", graph.list_query(Query::link_to(&potato)));
    println!("unlink cheese->danish {:?}", graph.unlink(&cheese, &danish));
    println!("unlink cheese->danish {:?}", graph.unlink(&cheese, &danish));
    println!("unlink cheese->potato {:?}", graph.unlink(&cheese, &potato));
    println!("query cheese->danish {:?}", graph.existence_query(Query::link_from_to(&cheese, &danish)));
    println!("query danish->cheese {:?}", graph.existence_query(Query::link_from_to(&danish, &cheese)));
    println!("query cheese->potato {:?}", graph.existence_query(Query::link_from_to(&cheese, &potato)));
    println!("query cheese->? {:?}", graph.existence_query(Query::link_from(&cheese)));
    println!("query danish->? {:?}", graph.existence_query(Query::link_from(&danish)));
    println!("query potato->? {:?}", graph.existence_query(Query::link_from(&potato)));
    println!("query ?->cheese {:?}", graph.existence_query(Query::link_to(&cheese)));
    println!("query ?->danish {:?}", graph.existence_query(Query::link_to(&danish)));
    println!("query ?->potato {:?}", graph.existence_query(Query::link_to(&potato)));
    graph.link(&banana, &danish);
    println!("serialized: {}", ron::ser::to_string_pretty(&graph, ron::ser::PrettyConfig::new()).unwrap());
    graph.delete(&banana);
    println!("serialized: {}", ron::ser::to_string_pretty(&graph, ron::ser::PrettyConfig::new()).unwrap());
}
