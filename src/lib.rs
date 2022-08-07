mod utils;
use std::{fs, path::Path};

use pest::{error::InputLocation, iterators::Pairs, Parser as P};
use pest_derive::Parser;
use wasm_bindgen::prelude::*;

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
    Command(String, Vec<AstNode>),
    Block(Vec<AstNode>),
    Identifier(String, Box<AstNode>),
    Env(String, Vec<AstNode>),
    FunctionCall(String, Box<AstNode>),
}

#[derive(Parser)]
#[grammar = "fuwa.pest"]
struct Parser;

fn ast_gen(pairs: Pairs<Rule>) -> Vec<AstNode> {
    let mut ast = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::program => {
                let children = ast_gen(pair.into_inner());

                ast.push(AstNode::Program(children));
            }
            Rule::NUMBER => {
                let number = pair.as_str().parse::<f64>().unwrap();
                ast.push(AstNode::Number(number));
            }
            Rule::STRING => {
                let string = pair.as_str().to_string().replace("\"", "").replace("'", "");
                ast.push(AstNode::String(string));
            }
            Rule::COMMENT => {
                let comment = pair.as_str().to_string().replace("//", "");
                ast.push(AstNode::Comment(comment));
            }
            Rule::COMMAND => {
                let tokens = pair.tokens();
                let command = tokens
                    .find(|t| t == Rule::command_keyword)
                    .unwrap()
                    .as_str();
                let children = ast_gen(pair.into_inner());

                ast.push(AstNode::Command(name, children));
            }
            Rule::BLOCK => {
                let mut children = ast_gen(pair.into_inner());

                ast.push(AstNode::Block(children));
            }
            Rule::identifier => {
                let mut children = ast_gen(pair.into_inner());
                println!("{:?}", children);
            }
            Rule::ENV => {
                let name = pair
                    .as_str()
                    .to_string()
                    .replace("env", "")
                    .replace(" ", "");
                let mut children = ast_gen(pair.into_inner());

                ast.push(AstNode::Env(name, children));
            }

            _ => {
                println!("{:?}", pair.tokens());
            }
        };
    }
    ast
}

#[wasm_bindgen]
pub fn parse(filename: String) {
    utils::set_panic_hook();

    let path = Path::new(&filename); // converts string to path
    let contents = fs::read_to_string(path).unwrap();
    let pairs = Parser::parse(Rule::program, &contents);

    match pairs {
        Ok(pairs) => {
            let ast = ast_gen(pairs);
            println!("{:?}", ast);
        }
        Err(e) => {
            console_error_panic_hook::hook(panic!("{}", e.with_path(path.to_str().unwrap())));
        }
    }
    // TODO - add error handling
    // TODO - add blocks
}
