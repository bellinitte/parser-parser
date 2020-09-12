mod builder;
mod parser;

pub fn parse(input: &str) -> String {
    use parser::syntax;

    format!("{:?}", syntax::<()>(&input).ok().map(|(_, grammar)| grammar))
}
