use ebnf_parser_parser::construct;

#[test]
fn test_ebnf() {
    assert!(construct(" abc = 'def'; ").is_ok());
    assert!(construct(" (* test *) ").is_err());
    assert!(construct(" (* test *").is_err());
    assert!(construct("a = b;;").is_err());
    assert!(construct("a = ;").is_ok());
}
