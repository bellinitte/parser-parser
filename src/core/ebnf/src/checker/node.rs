#[derive(Clone)]
pub enum Node {
    Nonterminal(String, Vec<Node>),
    Terminal(String),
}
