//! The entry point for Alto.

use alto_syntax::parseme::{Parser, Source};

fn main() {
    let src = std::fs::read_to_string("test.alto").unwrap();
    let mut input = Source::new(&src);
    let mut scanner = alto_syntax::scanner::new();

    for res in scanner.iter(&mut input) {
        println!("{:?}", res);
    }
}
