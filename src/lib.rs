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

#[derive(Parser)]
#[grammar = "fuwa.pest"]
struct Parser;

#[wasm_bindgen]
pub fn parse(filename: String) {
    let path = Path::new(&filename);
    let contents = fs::read_to_string(path).unwrap();
    let pairs = Parser::parse(Rule::program, &contents).unwrap();
    pairs.for_each(|pair| match pair.as_rule() {
        Rule::program => {
            println!("{:?}", pair);
        }
        _ => {
            println!("{:?}", pair);
        }
    });
}
