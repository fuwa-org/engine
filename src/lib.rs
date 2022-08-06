mod utils;
use std::{fs, path::Path};

use pest::{error::InputLocation, Parser as P};
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
    Identifier(String),
}

#[derive(Parser)]
#[grammar = "fuwa.pest"]
struct Parser;

#[wasm_bindgen]
pub fn parse(filename: String) {
    utils::set_panic_hook();
    let mut ast = Vec::<AstNode>::new();

    let path = Path::new(&filename); // converts string to path
    let contents = fs::read_to_string(path).unwrap();
    let pairs = Parser::parse(Rule::program, &contents);
    // TODO - add error handling
    // TODO - add blocks
    match pairs {
        Ok(pairs) => {
            pairs.for_each(|pair| match pair.as_rule() {
                Rule::COMMENT => {
                    let comment = pair.as_str().to_string();
                    ast.push(AstNode::Comment(comment));
                }
                Rule::STRING => {
                    let statement = pair.as_str().to_string();
                    ast.push(AstNode::String(statement));
                }
                Rule::NUMBER => {
                    let number = pair.as_str().parse::<f64>().unwrap();
                    ast.push(AstNode::Number(number));
                }
                Rule::COMMAND => {
                    println!("Command");
                    let command = pair.as_str().to_string();
                    let mut args = Vec::<AstNode>::new();
                    pair.into_inner().for_each(|pair| match pair.as_rule() {
                        Rule::identifier => {
                            let identifier = pair.as_str().to_string();
                            args.push(AstNode::String(identifier));
                        }
                        _ => (),
                    });
                    ast.push(AstNode::Command(command, args));
                }

                _ => {
                    println!("Other {}", pair.as_str());
                }
            });
        }
        Err(e) => {
            console_error_panic_hook::hook(panic!(
                "Error in {}:{}",
                filename,
                match e.location {
                    _ => e.to_string(),
                    InputLocation::Pos(pos) => pos.to_string(),
                    InputLocation::Span((start, end)) => format!("{}-{}", start, end),
                }
            ));
        }
    };
    println!("{:?}", ast);
}
