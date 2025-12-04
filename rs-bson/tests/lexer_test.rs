use std::fs::File;
use std::path::Path;

use rs_bson::lexer;

#[cfg(test)]
pub mod parser_tests {
    use crate::*;

    #[test]
    fn fail_invalid_header() {
        let input = Path::new("tests/test_data/invalid_header.bson");
        let file = File::open(input).unwrap();
        assert_eq!(lexer::lex(file), Err("Status: Fainted"));
    }

    #[test]
    fn fail_tab_character() {
        let input = Path::new("tests/test_data/invalid_tab_character.bson");
        let file = File::open(input).unwrap();
        assert_eq!(lexer::lex(file), Err("Poison Type: Tab character detected"));
    }

    #[test]
    fn fail_wrong_indentation() {
        let input = Path::new("tests/test_data/invalid_wrong_indentation.bson");
        let file = File::open(input).unwrap();
        assert_eq!(lexer::lex(file), Err("The attack missed!"));
    }

    #[test]
    fn fail_invalid_type() {
        let input = Path::new("tests/test_data/invalid_type.bson");
        let file = File::open(input).unwrap();
        assert_eq!(lexer::lex(file), Err("Target is immune!"));
    }
}
