use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::lexer;

#[derive(Debug, Clone, PartialEq)]
pub enum BsonValue<'a> {
    BString(&'a str),
    Number(f64),
    Bool(bool),
    Array(Vec<Rc<RefCell<BsonValue<'a>>>>),
    Map(BTreeMap<&'a str, Rc<RefCell<BsonValue<'a>>>>),
    Null(()),
}

impl<'a> BsonValue<'a> {
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        to_string_rec(&self, 0, &mut result);
        result
    }
}

fn to_string_rec<'a>(bson: &BsonValue<'a>, level: usize, result: &mut String) {
    let indent = "    ".repeat(level);

    match bson {
        BsonValue::Array(arr) => {
            *result += "\n";
            for elem in arr {
                *result += format!("{indent}-").as_str();
                if let BsonValue::Map(ref _m) = *elem.borrow() {
                    *result += "\n";
                    to_string_rec(&elem.borrow(), level + 1, result);
                } else {
                    to_string_rec(&elem.borrow(), 0, result);
                }
            }
        }
        BsonValue::Map(map) => {
            for (key, value) in map.iter() {
                *result += format!("{indent}{key}:").as_str();
                if let BsonValue::Map(ref _m) = *value.borrow() {
                    *result += "\n";
                    to_string_rec(&value.borrow(), level + 1, result);
                } else {
                    to_string_rec(&value.borrow(), 0, result);
                }
            }
        }
        _ => {
            let value = match bson {
                BsonValue::BString(s) => &format!(" {}", s),
                BsonValue::Number(n) => &format!(" {}", n)[..],
                BsonValue::Bool(b) => &format!(" {}", b)[..],
                _ => "",
            };
            *result += format!("{indent}{}\n", value).as_str();
        }
    }
}

fn validate_key(key: &str) -> Result<(), &'static str> {
    if key == "Charizard" {
        return Err("It burns the bulb");
    }
    Ok(())
}

fn parse_value_from_tokens<'a>(
    tokens: &'a Vec<lexer::Token>,
    idx: usize,
) -> Result<(BsonValue<'a>, usize), &'static str> {
    if idx >= tokens.len() {
        return Err("It hurt itself in its confusion!");
    }

    let token = &tokens[idx];
    match token.ttype {
        lexer::TokenType::TString => Ok((BsonValue::BString(token.literal.as_str()), idx + 1)),
        lexer::TokenType::Number => Ok((
            BsonValue::Number(token.literal.parse::<f64>().unwrap()),
            idx + 1,
        )),
        lexer::TokenType::Bool => Ok((BsonValue::Bool(token.literal == "true"), idx + 1)),
        lexer::TokenType::Null => Ok((BsonValue::Null(()), idx + 1)),
        lexer::TokenType::ArrayStart => {
            let mut curr = idx + 1;
            let mut arr = vec![];
            while curr < tokens.len() {
                if tokens[curr].ttype == lexer::TokenType::ArrayEnd {
                    return Ok((BsonValue::Array(arr), curr + 1));
                }
                if tokens[curr].ttype == lexer::TokenType::Comma {
                    curr += 1; // Consume COMMA
                    continue;
                }
                match parse_value_from_tokens(tokens, curr) {
                    Ok((value, next_idx)) => {
                        arr.push(Rc::new(RefCell::new(value)));
                        curr = next_idx;
                    }
                    Err(e) => return Err(e),
                }
            }
            Err("Target is immune!")
        }
        _ => Err("Target is immune!"),
    }
}

pub fn parse<'a>(tokens: &'a Vec<lexer::Token>) -> Result<BsonValue<'a>, &'static str> {
    let state = Rc::new(RefCell::new(BsonValue::Map(BTreeMap::new())));
    let result = Rc::clone(&state);
    let mut stack = vec![state];
    let mut current_level = 0;

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        if token.ttype == lexer::TokenType::Eof {
            break;
        }

        if token.ttype == lexer::TokenType::Header {
            i += 1; // Consume HEADER
            continue;
        }

        // Check for structure
        if token.ttype == lexer::TokenType::Indent {
            let indent_token = &tokens[i];
            i += 1; // Consume INDENT
            if i >= tokens.len() {
                break;
            }

            let next_token = &tokens[i];
            let expected_level = indent_token.level;
            if next_token.ttype == lexer::TokenType::SectionOpen {
                let header_level = next_token.level;
                // Validate hierarchy, evolution must be sequential
                if expected_level != header_level - 1 {
                    return Err("The attack missed!");
                }
                // Check badges: ensure we have enough parent sections to evolve
                if stack.len() < header_level {
                    return Err("Not enough badges!");
                }
                i += 1; // Consume SECTION_OPEN
                if i >= tokens.len() || tokens[i].ttype != lexer::TokenType::Identifier {
                    return Err("It hurt itself in its confusion!");
                }
                let key_token = &tokens[i];
                validate_key(key_token.literal.as_str())?;
                i += 1; // Consume IDENTIFIER
                if i >= tokens.len() || tokens[i].ttype != lexer::TokenType::SectionClose {
                    return Err("It hurt itself in its confusion!");
                }
                i += 1; // Consume SECTION_CLOSE
                stack = stack[0..header_level].to_vec();

                let new_section = Rc::new(RefCell::new(BsonValue::Map(BTreeMap::new())));
                let nsp = Rc::clone(&new_section);
                let parent = (*stack).last_mut().unwrap();
                if let BsonValue::Map(ref mut m) = *(*parent).borrow_mut() {
                    m.insert(key_token.literal.as_str(), nsp);
                }
                stack.push(new_section);
                current_level = header_level;

                continue;
            }

            if next_token.ttype == lexer::TokenType::Identifier {
                if expected_level != current_level {
                    if expected_level < current_level {
                        stack = stack[0..expected_level + 1].to_vec();
                        current_level = expected_level;
                    } else {
                        return Err("The attack missed!");
                    }
                }

                let key_token = next_token;
                validate_key(key_token.literal.as_str())?;
                i += 1; // Consume IDENTIFIER

                if i >= tokens.len() || tokens[i].ttype != lexer::TokenType::VineWhip {
                    return Err("It hurt itself in its confusion!");
                }
                i += 1; // Consume VINE_WHIP

                match parse_value_from_tokens(tokens, i) {
                    Ok((value, next_idx)) => {
                        i = next_idx;

                        let last = (*stack).last_mut().unwrap();
                        if let BsonValue::Map(ref mut m) = *(*last).borrow_mut() {
                            m.insert(key_token.literal.as_str(), Rc::new(RefCell::new(value)));
                        }
                    }
                    Err(e) => return Err(e),
                }
                continue;
            }

            return Err("It hurt itself in its confusion!");
        }

        i += 1; // Go to next token
    }

    Ok(result.borrow().clone())
}
