use ebnf_parser_parser::construct;

#[test]
fn test_ebnf() {
    assert!(construct(" abc = 'def'; ").is_ok());
    assert!(construct(" (* test *) ").is_err());
    assert!(construct(" (* test *").is_err());
    assert!(construct("a = b;;").is_err());
    assert!(construct("a = ;").is_ok());
    assert!(construct("twelve = \"1\", \"2\" ;").is_ok());
    assert!(construct("natural number = digit excluding zero, { digit };").is_ok());
    assert!(construct("integer = \"0\" | [ \"-\" ], natural number ;").is_ok());
    assert!(construct("lhs = \"\";").is_err());
}
