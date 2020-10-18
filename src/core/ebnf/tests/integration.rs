use ebnf_parser_parser::parse;

#[test]
fn test_ebnf() {
    assert!(parse(" abc = 'def'; ").is_ok());
    assert!(parse(" (* test *) ").is_err());
    assert!(parse(" (* test *").is_err());
    assert!(parse("a = b;;").is_err());
    assert!(parse("a = ;").is_ok());
}
