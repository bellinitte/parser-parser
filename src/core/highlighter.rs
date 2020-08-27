pub enum Span {
    Keyword(usize),
    Number(usize),
    String(usize),
    Punctuation(usize),
    Operator(usize),
    Comment(usize),
}

trait HtmlHighlight {
    fn highlight(&self, input: &str) -> String;
}

impl HtmlHighlight for Vec<Span> {
    fn highlight(&self, input: &str) -> String {
        unimplemented!()
    }
}

pub fn parse(input: &str) -> Vec<Span> {
    unimplemented!()
}
