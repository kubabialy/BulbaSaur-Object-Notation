use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Header,
    Indent,
    SectionOpen,
    SectionClose,
    Identifier,
    VineWhip,
    TString,
    Number,
    Bool,
    Null,
    ArrayStart,
    ArrayEnd,
    Comma,
    Eof,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
    line: usize,
    pub level: usize,
}

fn count_whitespaces_at_start(input: &str) -> usize {
    input
        .chars()
        .take_while(|ch| ch.is_whitespace() && *ch != '\n')
        .map(|ch| ch.len_utf8())
        .sum()
}

fn tokenize_value(
    value: &str,
    line_num: usize,
    tokens: &mut Vec<Token>,
) -> Result<(), &'static str> {
    if value.is_empty() {
        return Ok(());
    }

    // String literal
    if value.starts_with("\"") && value.ends_with("\"") {
        tokens.push(Token {
            ttype: TokenType::TString,
            literal: value[1..value.len() - 1].to_string(),
            line: line_num,
            level: 0,
        });
        return Ok(());
    }

    // Bool true
    if value == "SuperEffective" {
        tokens.push(Token {
            ttype: TokenType::Bool,
            literal: String::from("true"),
            line: line_num,
            level: 0,
        });
        return Ok(());
    }
    // Bool false
    if value == "NotVeryEffective" {
        tokens.push(Token {
            ttype: TokenType::Bool,
            literal: String::from("false"),
            line: line_num,
            level: 0,
        });
        return Ok(());
    }

    // Null
    if value == "MissingNo" {
        tokens.push(Token {
            ttype: TokenType::Null,
            literal: String::from(""),
            line: line_num,
            level: 0,
        });
        return Ok(());
    }

    // Array <| ... |>
    if value.starts_with("<|") && value.ends_with("|>") {
        tokens.push(Token {
            ttype: TokenType::ArrayStart,
            literal: String::from(""),
            line: line_num,
            level: 0,
        });
        let array_content = value[2..value.len() - 2].trim();
        if !array_content.is_empty() {
            let elements = array_content.split(',');
            for (i, elem) in elements.enumerate() {
                if i > 0 {
                    tokens.push(Token {
                        ttype: TokenType::Comma,
                        literal: String::from(""),
                        line: line_num,
                        level: 0,
                    });
                }
                tokenize_value(elem.trim(), line_num, tokens)?;
            }
        }
        tokens.push(Token {
            ttype: TokenType::ArrayEnd,
            literal: String::from(""),
            line: line_num,
            level: 0,
        });
        return Ok(());
    }

    // Number
    if value.parse::<f64>().is_ok() {
        tokens.push(Token {
            ttype: TokenType::Number,
            literal: value.to_string(),
            line: line_num,
            level: 0,
        });
        return Ok(());
    }

    Err("Target is immune!")
}

fn tokenize_line(
    line: &mut str,
    line_num: usize,
    tokens: &mut Vec<Token>,
) -> Result<(), &'static str> {
    // Evolution stage: (o) key (o)
    if line.starts_with("(o) ") && line.ends_with(" (o)") {
        tokens.push(Token {
            ttype: TokenType::SectionOpen,
            literal: String::from(""),
            line: line_num,
            level: 1,
        });
        tokens.push(Token {
            ttype: TokenType::Identifier,
            literal: line[4..line.len() - 4].to_string(),
            line: line_num,
            level: 1,
        });
        tokens.push(Token {
            ttype: TokenType::SectionClose,
            literal: String::from(""),
            line: line_num,
            level: 1,
        });
        return Ok(());
    }
    if line.starts_with("(O) ") && line.ends_with(" (O)") {
        tokens.push(Token {
            ttype: TokenType::SectionOpen,
            literal: String::from(""),
            line: line_num,
            level: 2,
        });
        tokens.push(Token {
            ttype: TokenType::Identifier,
            literal: line[4..line.len() - 4].to_string(),
            line: line_num,
            level: 2,
        });
        tokens.push(Token {
            ttype: TokenType::SectionClose,
            literal: String::from(""),
            line: line_num,
            level: 2,
        });
        return Ok(());
    }
    if line.starts_with("(@) ") && line.ends_with(" (@)") {
        tokens.push(Token {
            ttype: TokenType::SectionOpen,
            literal: String::from(""),
            line: line_num,
            level: 3,
        });
        tokens.push(Token {
            ttype: TokenType::Identifier,
            literal: line[4..line.len() - 4].to_string(),
            line: line_num,
            level: 3,
        });
        tokens.push(Token {
            ttype: TokenType::SectionClose,
            literal: String::from(""),
            line: line_num,
            level: 3,
        });
        return Ok(());
    }

    // Vine whip: key ~~~> value
    let re = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)\s*(~{1,}>)\s*(.*)$").unwrap();
    match re.captures(line) {
        Some(matches) => {
            tokens.push(Token {
                ttype: TokenType::Identifier,
                literal: matches.get(1).unwrap().as_str().to_string(),
                line: line_num,
                level: 0,
            });
            tokens.push(Token {
                ttype: TokenType::VineWhip,
                literal: String::from(""),
                line: line_num,
                level: 0,
            });

            let value = matches.get(3).unwrap().as_str().trim();
            tokenize_value(value, line_num, tokens)
        }
        None => Err("It hurt itself in its confusion!"),
    }
}

pub fn lex(file: File) -> Result<Vec<Token>, &'static str> {
    let mut tokens: Vec<Token> = vec![];
    let mut line_num = 0;
    let reader = BufReader::new(file);

    for line_r in reader.lines() {
        let mut line = line_r.unwrap();

        // First line: check header
        if line_num == 0 {
            if line != "BULBA!" {
                return Err("Status: Fainted");
            }
            tokens.push(Token {
                ttype: TokenType::Header,
                literal: line.clone(),
                line: 1,
                level: 0,
            });
            line_num += 1;
            continue;
        }
        line_num += 1;

        // Sleep powder: ignore comments
        if let Some(comment_idx) = line.find("zZz") {
            line.truncate(comment_idx);
        }

        // Poison powder: tab character not allowed!
        if line.contains("\t") {
            return Err("Poison Type: Tab character detected");
        }

        line = line.trim_end().to_string();
        if line.is_empty() {
            continue;
        }

        // Solar beam: check indentation is multiple of 4
        let indent = count_whitespaces_at_start(&line);
        if !indent.is_multiple_of(4) {
            return Err("The attack missed!");
        }
        let level = indent / 4;
        tokens.push(Token {
            ttype: TokenType::Indent,
            literal: String::from(""),
            line: line_num,
            level,
        });

        line = line.trim().to_string();
        tokenize_line(&mut line, line_num, &mut tokens)?;
    }

    tokens.push(Token {
        ttype: TokenType::Eof,
        literal: String::from(""),
        line: line_num,
        level: 0,
    });
    Ok(tokens)
}
