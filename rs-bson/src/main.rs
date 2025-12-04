use std::env;
use std::fs::File;
use std::path::Path;

mod lexer;
mod parser;

fn main() {
    let args: Vec<_> = env::args().collect();
    let input = if args.len() == 2 {
        Path::new(&args[1])
    } else {
        Path::new("tests/test_data/main_input.bson")
    };
    let file = File::open(input).unwrap();
    let tokens = lexer::lex(file).unwrap();
    let res = parser::parse(&tokens).unwrap();
    print!("{}", res.to_string());
}
