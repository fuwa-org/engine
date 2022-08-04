mod utils;
use std::{fs, path::Path};

use pest::Parser as P;
use pest_derive::Parser;
use wasm_bindgen::{convert::WasmAbi, describe::WasmDescribe, prelude::*};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[derive(Debug)]
pub enum AstNode {
    Program(Vec<AstNode>),
    Number(f64),
    String(String),
    Comment(String),
}

#[derive(Parser)]
#[grammar = "fuwa.pest"]
struct Parser;

#[wasm_bindgen]
pub fn parse(filename: String) {
    let mut ast = Vec::<AstNode>::new();

    let path = Path::new(&filename); // converts string to path
    let contents = fs::read_to_string(path).unwrap();
    let pairs = Parser::parse(Rule::program, &contents).unwrap();
    // TODO - add error handling
    // TODO - add blocks
    pairs.for_each(|pair| match pair.as_rule() {
        Rule::COMMENT => {
            let comment = pair.as_str().to_string();
            ast.push(AstNode::Comment(comment));
        }
        Rule::STRING => {
            let statement = pair.as_str().to_string();
            ast.push(AstNode::String(statement));
        }
        _ => {
            println!("Other {}", pair.as_str());
        }
    });
    println!("{:?}", ast);
}
