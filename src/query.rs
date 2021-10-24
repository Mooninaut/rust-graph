#[derive(Debug)]
pub enum Query {
    LinkFromTo(String, String),
    Node(String),
    LinkFrom(String),
    LinkTo(String),
}
