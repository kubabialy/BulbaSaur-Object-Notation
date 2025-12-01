use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::RefCell;
use regex::Regex;

// ================ LEXER ================ //

#[derive(Debug,PartialEq,Clone)]
enum TokenType {
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
#[derive(Debug,Clone)]
struct Token {
  ttype: TokenType,
  literal: String,
  line: usize,
  level: usize,
}

fn count_whitespaces_at_start(input: &str) -> usize {
  input
    .chars()
    .take_while(|ch| ch.is_whitespace() && *ch != '\n')
    .map(|ch| ch.len_utf8())
    .sum()
}

fn tokenize_value(value: &str, line_num: usize, tokens: &mut Vec<Token>) -> Result<(), &'static str> {
  if value.is_empty() {
    return Ok(());
  }

  // String literal
  if value.starts_with("\"") && value.ends_with("\"") {
    tokens.push(Token{
      ttype: TokenType::TString,
      literal: value[1..value.len()-1].to_string(),
      line: line_num,
      level: 0,
    });
    return Ok(());
  }

  // Bool true
  if value == "SuperEffective" {
    tokens.push(Token{
      ttype: TokenType::Bool,
      literal: String::from("true"),
      line: line_num,
      level: 0,
    });
    return Ok(());
  }
  // Bool false
  if value == "NotVeryEffective" {
    tokens.push(Token{
      ttype: TokenType::Bool,
      literal: String::from("false"),
      line: line_num,
      level: 0,
    });
    return Ok(());
  }

  // Null
  if value == "MissingNo" {
    tokens.push(Token{
      ttype: TokenType::Null,
      literal: String::from(""),
      line: line_num,
      level: 0,
    });
    return Ok(());
  }

  // Array <| ... |>
  if value.starts_with("<|") && value.ends_with("|>") {
    tokens.push(Token{
      ttype: TokenType::ArrayStart,
      literal: String::from(""),
      line: line_num,
      level: 0,
    });
    let array_content = value[2..value.len()-2].trim();
    if !array_content.is_empty() {
      let elements = array_content.split(',');
      for (i, elem) in elements.enumerate() {
        if i > 0 {
          tokens.push(Token{
            ttype: TokenType::Comma,
            literal: String::from(""),
            line: line_num,
            level: 0,
          });
        }
        tokenize_value(elem.trim(), line_num, tokens)?;
      }
    }
    tokens.push(Token{
      ttype: TokenType::ArrayEnd,
      literal: String::from(""),
      line: line_num,
      level: 0,
    });
    return Ok(());
  }

  // Number
  if value.parse::<f64>().is_ok() {
    tokens.push(Token{
      ttype: TokenType::Number,
      literal: value.to_string(),
      line: line_num,
      level: 0,
    });
    return Ok(());
  }

  Err("Target is immune!")
}

fn tokenize_line(line: &mut str, line_num: usize, tokens: &mut Vec<Token>) -> Result<(), &'static str> {
  // Evolution stage: (o) key (o)
  if line.starts_with("(o) ") && line.ends_with(" (o)") {
    tokens.push(Token{
      ttype: TokenType::SectionOpen,
      literal: String::from(""),
      line: line_num,  
      level: 1,
    });
    tokens.push(Token{
      ttype: TokenType::Identifier,
      literal: line[4..line.len()-4].to_string(),
      line: line_num,  
      level: 1,
    });
    tokens.push(Token{
      ttype: TokenType::SectionClose,
      literal: String::from(""),
      line: line_num,  
      level: 1,
    });
    return Ok(());
  }
  if line.starts_with("(O) ") && line.ends_with(" (O)") {
    tokens.push(Token{
      ttype: TokenType::SectionOpen,
      literal: String::from(""),
      line: line_num,  
      level: 2,
    });
    tokens.push(Token{
      ttype: TokenType::Identifier,
      literal: line[4..line.len()-4].to_string(),
      line: line_num,  
      level: 2,
    });
    tokens.push(Token{
      ttype: TokenType::SectionClose,
      literal: String::from(""),
      line: line_num,  
      level: 2,
    });
    return Ok(());
  }
  if line.starts_with("(@) ") && line.ends_with(" (@)") {
    tokens.push(Token{
      ttype: TokenType::SectionOpen,
      literal: String::from(""),
      line: line_num,  
      level: 3,
    });
    tokens.push(Token{
      ttype: TokenType::Identifier,
      literal: line[4..line.len()-4].to_string(),
      line: line_num,  
      level: 3,
    });
    tokens.push(Token{
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
      tokens.push(Token{
        ttype: TokenType::Identifier,
        literal: matches.get(1).unwrap().as_str().to_string(),
        line: line_num,
        level: 0,
      });
      tokens.push(Token{
        ttype: TokenType::VineWhip,
        literal: String::from(""),
        line: line_num,
        level: 0,
      });

      let value = matches.get(3).unwrap().as_str().trim();
      tokenize_value(value, line_num, tokens)
    },
    None => Err("It hurt itself in its confusion!"),
  }
}

fn lex(file: File) -> Result<Vec<Token>, &'static str> {
  let mut tokens:Vec<Token> = vec![];
  let mut line_num = 0;
  let reader = BufReader::new(file);

  for line_r in reader.lines() {
    let mut line = line_r.unwrap();

    // First line: check header
    if line_num == 0 {
      if line != "BULBA!" {
        return Err("Status: Fainted");
      }
      tokens.push(Token{
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
    tokens.push(Token{
      ttype: TokenType::Indent,
      literal: String::from(""),
      line: line_num,
      level,
    });

    line = line.trim().to_string();    
    tokenize_line(&mut line, line_num, &mut tokens)?;
  }

  tokens.push(Token{
    ttype: TokenType::Eof,
    literal: String::from(""),
    line: line_num,
    level: 0,
  });
  Ok(tokens)
}

// ================ PARSER ================ //
#[allow(dead_code)]
#[derive(Debug,Clone)]
enum BsonValue<'a> {
  BString(&'a str),
  Number(f64),
  Bool(bool),
  Array(Vec<Rc<RefCell<BsonValue<'a>>>>),
  Map(BTreeMap<&'a str, Rc<RefCell<BsonValue<'a>>>>),
  Null(()),
}

fn validate_key(key: &str) -> Result<(), &'static str> {
  if key == "Charizard" {
    return Err("It burns the bulb")
  }
  Ok(())
}

fn parse_value_from_tokens<'a>(tokens: &'a Vec<Token>, idx: usize) -> Result<(BsonValue<'a>, usize), &'static str> {
  if idx >= tokens.len() {
    return Err("It hurt itself in its confusion!");
  }

  let token = &tokens[idx];
  match token.ttype {
    TokenType::TString => Ok((BsonValue::BString(token.literal.as_str()), idx+1)),
    TokenType::Number => Ok((BsonValue::Number(token.literal.parse::<f64>().unwrap()), idx+1)),
    TokenType::Bool => Ok((BsonValue::Bool(token.literal == "true"), idx+1)),
    TokenType::Null => Ok((BsonValue::Null(()), idx+1)),
    TokenType::ArrayStart => {
      let mut curr = idx + 1;
      let mut arr = vec![];
      while curr < tokens.len() {
        if tokens[curr].ttype == TokenType::ArrayEnd {
          return Ok((BsonValue::Array(arr), curr+1));
        }
        if tokens[curr].ttype == TokenType::Comma {
          curr += 1;
          continue;
        }
        match parse_value_from_tokens(tokens, curr) {
          Ok((value, next_idx)) => {
            arr.push(Rc::new(RefCell::new(value)));
            curr = next_idx;
          },
          Err(e) => return Err(e),
        }
      }
      Err("Target is immune!")
    },
    _ => Err("Target is immune!"),
  }
}

fn parse<'a>(tokens: &'a Vec<Token>) -> Result<BsonValue<'a>, &'static str> {
  let state = Rc::new(RefCell::new(BsonValue::Map(BTreeMap::new())));
  let result = Rc::clone(&state);
  let mut stack = vec![state];
  let mut current_level = 0;

  let mut i = 0;
  while i < tokens.len() {
    let token = &tokens[i];
    if token.ttype == TokenType::Eof {
      break;
    }
    if token.ttype == TokenType::Header {
      i += 1;
      continue;
    }
    
    // Check for structure
    if token.ttype == TokenType::Indent {
      let indent_token = &tokens[i];
      i += 1;
      if i >= tokens.len() {
        break;
      }

      let next_token = &tokens[i];
      let expected_level = indent_token.level;
      if next_token.ttype == TokenType::SectionOpen {
        let header_level = next_token.level;
        // Validate hierarchy, evolution must be sequential
        if expected_level != header_level - 1 {
          return Err("The attack missed!");
        }
        // Check badges: ensure we have enough parent sections to evolve
        if stack.len() < header_level {
          return Err("Not enough badges!");
        }
        i += 1;
        if i >= tokens.len() || tokens[i].ttype != TokenType::Identifier {
          return Err("It hurt itself in its confusion!");
        }
        let key_token = &tokens[i];
        validate_key(key_token.literal.as_str())?;
        i += 1;
        if i >= tokens.len() || tokens[i].ttype != TokenType::SectionClose {
          return Err("It hurt itself in its confusion!");
        }
        i += 1;
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

      if next_token.ttype == TokenType::Identifier {
        if expected_level != current_level {
          if expected_level < current_level {
            stack = stack[0..expected_level+1].to_vec();
            current_level = expected_level;
          } else {
            return Err("The attack missed!");
          }
        }

        let key_token = next_token;
        validate_key(key_token.literal.as_str())?; 
        i += 1;

        if i >= tokens.len() || tokens[i].ttype != TokenType::VineWhip {
          return Err("It hurt itself in its confusion!");
        }
        i += 1;

        match parse_value_from_tokens(tokens, i) {
          Ok((value, next_idx)) => {
            i = next_idx;
            
            let last = (*stack).last_mut().unwrap();
            if let BsonValue::Map(ref mut m) = *(*last).borrow_mut() {
              m.insert(key_token.literal.as_str(), Rc::new(RefCell::new(value)));
            }
          },
          Err(e) => return Err(e),
        }
        continue;
      }

      return Err("It hurt itself in its confusion!");
    }

    i += 1;
  }

  Ok(result.borrow().clone())
} 

fn print_bson<'a>(bson: &BsonValue<'a>) {
  print_bson_rec(bson, 0);  
}

fn print_bson_rec<'a>(bson: &BsonValue<'a>, level: usize) {
  let indent = "  ".repeat(level);
  
  match bson {
    BsonValue::Array(arr) => {
      println!();
      for elem in arr {
        print!("{indent}- ");
        if let BsonValue::Map(ref _m) = *elem.borrow() {
          println!();
          print_bson_rec(&elem.borrow(), level+1);
        } else {
          print_bson_rec(&elem.borrow(), 0);
        }
      }
    },
    BsonValue::Map(map) => {
      for (key, value) in map.iter() {
        print!("{indent}{key}: ");
        if let BsonValue::Map(ref _m) = *value.borrow() {
          println!();
          print_bson_rec(&value.borrow(), level+1);
        } else {
          print_bson_rec(&value.borrow(), 0);
        }
      }
    },
    _ => {
      let value = match bson {
        BsonValue::BString(s) => s,
        BsonValue::Number(n) => &format!("{}", n)[..],
        BsonValue::Bool(b) => &format!("{}", b)[..],
        _ => "",
      };
      println!("{indent}{}", value);
    }    
  }
} 

fn main() {
  let input = Path::new("input.txt");
  let file = File::open(input).unwrap();
  let tokens = lex(file).unwrap();
  let res = parse(&tokens).unwrap();
  // println!("FINAL RESULT: {:?}\n\n", res);
  print_bson(&res);
}
