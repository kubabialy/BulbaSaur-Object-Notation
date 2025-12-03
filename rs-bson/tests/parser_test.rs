use std::fs::File;
use std::path::Path;

use rs_bson::lexer;
use rs_bson::parser;

#[cfg(test)]
pub mod parser_tests {
    use crate::*;

    #[test]
    fn parse_valid() {
        let input = Path::new("tests/test_data/valid.bson");
        let file = File::open(input).unwrap();
        let tokens = lexer::lex(file).unwrap();
        let parsed = parser::parse(&tokens).unwrap();
        let expected = "app_name: Pokedex_API
database:
    host: 127.0.0.1
    pool:
        KERNEL_FLAGS:
            panic_on_fail: true
        max_connections: 100
is_production: false
missing_data:
version: 1.5
whitelist:
- Prof_Oak
- Mom
";
        assert_eq!(parsed.to_string(), expected);
    }

    #[test]
    fn fail_charizard() {
        let input = Path::new("tests/test_data/invalid_charizard.bson");
        let file = File::open(input).unwrap();
        let tokens = lexer::lex(file).unwrap();
        assert_eq!(parser::parse(&tokens), Err("It burns the bulb"));
    }

    #[test]
    fn fail_invalid_nesting() {
        let input = Path::new("tests/test_data/invalid_nesting.bson");
        let file = File::open(input).unwrap();
        let tokens = lexer::lex(file).unwrap();
        assert_eq!(parser::parse(&tokens), Err("Not enough badges!"));
    }
}
