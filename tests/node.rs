use std::{env::current_dir, path::Path};

use fuwa_engine::*;

#[test]
fn test_parse() {
    parse(
        current_dir()
            .unwrap()
            .join("tests/example.fuwa")
            .to_str()
            .unwrap()
            .to_string(),
    );
}
