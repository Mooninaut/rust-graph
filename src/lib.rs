pub mod graph;
pub mod query;
pub mod value;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::graph::Graph;
    use crate::graph::NodeKey;
    use crate::query::Query;

    pub fn get_graph() -> Graph<String> {
        let mut graph: Graph<String> = Graph::new();
        assert_eq!(true, graph.insert(cheese(), 64.0f64/0.99f64));
        assert_eq!(true, graph.insert(danish(), "fancy"));
        assert_eq!(true, graph.insert(banana(), 7u64));

        assert_eq!(Some(true), graph.link(&cheese(), &danish()));
        assert_eq!(Some(false), graph.link(&cheese(), &danish()));
        assert_eq!(None, graph.link(&cheese(), &potato()));

        graph
    }

    pub fn cheese() -> String {
        "cheese".into()
    }

    pub fn danish() -> String {
        "danish".into()
    }

    pub fn banana() -> String {
        "banana".into()
    }

    pub fn potato() -> String {
        "potato".into()
    }

    fn print_graph<T>(graph: &Graph<T>)
            where T: NodeKey + serde::Serialize {
        let pretty_config = ron::ser::PrettyConfig::new();

        println!("serialized: {}", ron::ser::to_string_pretty(graph, pretty_config).unwrap());
    }

    #[test]
    fn should_query_nodes() {
        let graph = get_graph();

        assert_eq!(Some(true), graph.query_node(&banana()).exists());
        assert_eq!(Some(true), graph.query_node(&cheese()).exists());
        assert_eq!(Some(true), graph.query_node(&danish()).exists());
        assert_eq!(Some(false), graph.query_node(&potato()).exists());
    }

    #[test]
    fn should_exist_query_link_source_target() {
        let graph = get_graph();

        let cheese = cheese();
        let danish = danish();

        assert_eq!(Some(true) , graph.query_link_source(&cheese).target(&danish).exists());
        assert_eq!(Some(true) , graph.query_link_target(&danish).source(&cheese).exists());
        assert_eq!(Some(false), graph.query_link_source(&danish).target(&cheese).exists());
        assert_eq!(None       , graph.query_link_target(&potato()).source(&cheese).exists());
    }

    #[test]
    fn should_exist_query_link_source() {
        let graph = get_graph();

        assert_eq!(Some(true ), graph.query_link_source(&cheese()).exists());
        assert_eq!(Some(false), graph.query_link_source(&danish()).exists());
        assert_eq!(None       , graph.query_link_source(&potato()).exists());
    }

    #[test]
    fn should_exist_query_link_target() {
        let graph = get_graph();

        assert_eq!(Some(true ), graph.existence_query(&Query::link_target(&danish())));
        assert_eq!(Some(false), graph.existence_query(&Query::link_target(&cheese())));
        assert_eq!(None       , graph.existence_query(&Query::link_target(&potato())));

    }

    #[test]
    fn should_set_query_link_source() {
        let graph = get_graph();

        assert_eq!(Ok(HashSet::from([danish()])),
            graph.query_link_source(&cheese()).as_set());
        assert_eq!(Ok(HashSet::new()),
            graph.query_link_source(&danish()).as_set());
        assert_eq!(Err("Node '\"potato\"' does not exist".to_string()),
            graph.query_link_source(&potato()).as_set());
    }

    #[test]
    fn should_set_query_link_target() {
        let graph = get_graph();

        assert_eq!(Ok(HashSet::new()),
            graph.set_query(&Query::link_target(&cheese())));
        assert_eq!(Ok(HashSet::from([cheese()])),
            graph.set_query(&Query::link_target(&danish())));
        assert_eq!(Err("Node '\"potato\"' does not exist".to_string()),
            graph.set_query(&Query::link_target(&potato())));
    }

    #[test]
    fn should_do_test_grab_bag() {
        let mut graph = get_graph();

        print_graph(&graph);

        assert_eq!(Some(true), graph.unlink(&cheese(), &danish()));
        assert_eq!(Some(false), graph.unlink(&cheese(), &danish()));
        assert_eq!(None, graph.unlink(&cheese(), &potato()));

        assert_eq!(Some(false), graph.existence_query(&Query::link_source_target(&cheese(), &danish())));
        assert_eq!(Some(false), graph.existence_query(&Query::link_source_target(&danish(), &cheese())));
        assert_eq!(None, graph.existence_query(&Query::link_source_target(&cheese(), &potato())));

        assert_eq!(Some(true), graph.existence_query(&Query::link_source(&cheese())));
        assert_eq!(Some(false), graph.existence_query(&Query::link_source(&danish())));
        assert_eq!(None, graph.existence_query(&Query::link_source(&potato())));
        assert_eq!(Some(true), graph.existence_query(&Query::link_target(&danish())));
        assert_eq!(Some(false), graph.existence_query(&Query::link_target(&cheese())));
        assert_eq!(None, graph.existence_query(&Query::link_target(&potato())));

        assert_eq!(Some(true), graph.link(&banana(), &danish()));

        print_graph(&graph);

        assert_eq!(true, graph.delete(&banana()));

        print_graph(&graph);
    }
}
