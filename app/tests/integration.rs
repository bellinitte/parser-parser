#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use ebnf::parse;

#[wasm_bindgen_test]
fn test_ebnf() {
    assert!(parse(" abc = 'def'; ").is_ok());
    assert!(parse(" (* test *) ").is_err());
    assert!(parse(" (* test *").is_err());
    assert!(parse("a = b;;").is_err());
    assert!(parse("a = ;").is_ok());
}
