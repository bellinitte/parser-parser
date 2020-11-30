pub mod error;

use super::ebnf::checker::node::Node;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;

#[allow(unused_unsafe)]
pub fn tree(node: Node) -> Object {
    match node {
        Node::Terminal(string) => {
            let obj = Object::new();
            unsafe {
                Reflect::set(&obj, &"name".into(), &format!("\"{}\"", string).into())
                    .unwrap();
            }
            return obj;
        },
        Node::Nonterminal(name, nodes) => {
            let obj = Object::new();
            let children: Vec<Object> = nodes.iter().cloned().map(tree).collect();
            let children_array: Array = children.into_iter().map(JsValue::from).collect();
            unsafe {
                Reflect::set(&obj, &"name".into(), &name.into())
                    .unwrap();
                Reflect::set(&obj, &"children".into(), &children_array.into())
                        .unwrap();
            }
            return obj;
        }
    }
}
