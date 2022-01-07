pub mod graph;
pub mod query;
pub mod value;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::graph::Graph;
    use crate::graph::NodeKey;
    use crate::value::Value;
    use crate::query::Query;

    pub fn get_graph() -> Graph<Box<str>> {
        let mut graph: Graph<Box<str>> = Graph::new();
        assert_eq!(true, graph.insert(cheese(), Value::Float64(64.0/0.99)));
        assert_eq!(true, graph.insert(danish(), Value::from("fancy")));
        assert_eq!(true, graph.insert(banana(), 7u64));

        assert_eq!(Some(true), graph.link(&cheese(), &danish()));
        assert_eq!(Some(false), graph.link(&cheese(), &danish()));
        assert_eq!(None, graph.link(&cheese(), &potato()));

        graph
    }

    pub fn cheese() -> Box<str> {
        "cheese".into()
    }

    pub fn danish() -> Box<str> {
        "danish".into()
    }

    pub fn banana() -> Box<str> {
        "banana".into()
    }

    pub fn potato() -> Box<str> {
        "potato".into()
    }

    fn print_graph<T>(graph: &Graph<T>)
            where T: NodeKey + serde::Serialize {
        let pretty_config = ron::ser::PrettyConfig::new();

        println!("serialized: {}", ron::ser::to_string_pretty(graph, pretty_config).unwrap());
    }

    #[test]
    fn should_exist_query_link_from_to() {
        let graph = get_graph();

        let cheese = cheese();
        let danish = danish();

        assert_eq!(Some(true) , graph.existence_query(&Query::link_from_to(&cheese, &danish)));
        assert_eq!(Some(false), graph.existence_query(&Query::link_from_to(&danish, &cheese)));
        assert_eq!(None       , graph.existence_query(&Query::link_from_to(&cheese, &potato())));
    }

    #[test]
    fn should_exist_query_link_from() {
        let graph = get_graph();

        assert_eq!(Some(true ), graph.existence_query(&Query::link_from(&cheese())));
        assert_eq!(Some(false), graph.existence_query(&Query::link_from(&danish())));
        assert_eq!(None       , graph.existence_query(&Query::link_from(&potato())));
    }

    #[test]
    fn should_exist_query_link_to() {
        let graph = get_graph();

        assert_eq!(Some(true ), graph.existence_query(&Query::link_to(&danish())));
        assert_eq!(Some(false), graph.existence_query(&Query::link_to(&cheese())));
        assert_eq!(None       , graph.existence_query(&Query::link_to(&potato())));

    }

    #[test]
    fn should_set_query_link_from() {
        let graph = get_graph();

        assert_eq!(Ok(HashSet::from([danish()])),
            graph.set_query(&Query::link_from(&cheese())));
        assert_eq!(Ok(HashSet::new()),
            graph.set_query(&Query::link_from(&danish())));
        assert_eq!(Err("Node '\"potato\"' does not exist".to_string()),
            graph.set_query(&Query::link_from(&potato())));
    }

    #[test]
    fn should_set_query_link_to() {
        let graph = get_graph();

        assert_eq!(Ok(HashSet::new()),
            graph.set_query(&Query::link_to(&cheese())));
        assert_eq!(Ok(HashSet::from([cheese()])),
            graph.set_query(&Query::link_to(&danish())));
        assert_eq!(Err("Node '\"potato\"' does not exist".to_string()),
            graph.set_query(&Query::link_to(&potato())));
    }

    #[test]
    fn should_do_test_grab_bag() {
        let mut graph = get_graph();

        print_graph(&graph);

        assert_eq!(Some(true), graph.unlink(&cheese(), &danish()));
        assert_eq!(Some(false), graph.unlink(&cheese(), &danish()));
        assert_eq!(None, graph.unlink(&cheese(), &potato()));

        assert_eq!(Some(false), graph.existence_query(&Query::link_from_to(&cheese(), &danish())));
        assert_eq!(Some(false), graph.existence_query(&Query::link_from_to(&danish(), &cheese())));
        assert_eq!(None, graph.existence_query(&Query::link_from_to(&cheese(), &potato())));

        assert_eq!(Some(true), graph.existence_query(&Query::link_from(&cheese())));
        assert_eq!(Some(false), graph.existence_query(&Query::link_from(&danish())));
        assert_eq!(None, graph.existence_query(&Query::link_from(&potato())));
        assert_eq!(Some(true), graph.existence_query(&Query::link_to(&danish())));
        assert_eq!(Some(false), graph.existence_query(&Query::link_to(&cheese())));
        assert_eq!(None, graph.existence_query(&Query::link_to(&potato())));

        assert_eq!(Some(true), graph.link(&banana(), &danish()));

        print_graph(&graph);

        assert_eq!(true, graph.delete(&banana()));

        print_graph(&graph);
    }
}