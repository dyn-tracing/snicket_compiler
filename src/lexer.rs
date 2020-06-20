extern crate regex;
use self::regex::Regex;

lazy_static! {
  static ref TOKENS      : Regex = Regex::new(r"[0-9]+|[A-Za-z_][A-Za-z0-9_]*|<-|->|-|\*|\{|\}|\(|\)|\[|\]|:|-|\*|\.\.|\||\S+").unwrap();
  static ref IDENTIFIERS : Regex = Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*$").unwrap();
  static ref VALUES      : Regex = Regex::new(r"^([0-9]+)$").unwrap();
}

use token::Token;
fn get_single_token(tok_str : &str) -> Token {
  if IDENTIFIERS.is_match(tok_str) {
    return Token::Identifier(tok_str);
  } else if VALUES.is_match(tok_str) {
    return Token::Value(tok_str.parse::<u32>().unwrap());
  } else {
    return match tok_str {
      ":" => Token::Colon,
      ";" => Token::SemiColon,
      "," => Token::Comma,
      "->"=> Token::FwdArrow,
      "<-"=> Token::BwdArrow,

      "(" => Token::ParenLeft,
      ")" => Token::ParenRight,
      "[" => Token::SquareLeft,
      "]" => Token::SquareRight,
      "{" => Token::BraceLeft,
      "}" => Token::BraceRight,

      "-" => Token::Relationship,
      "*" => Token::Iterator,
      ".."=> Token::Ellipsis,
      "|" => Token::Pipe,

      _   => panic!("Unrecognized token string: {}", tok_str)
    }
  }
}

pub fn get_tokens(input_program : &str) -> Vec<Token> {
  let mut token_array = Vec::new();
  for cap in TOKENS.captures_iter(input_program) {
    let ref tok_str = cap.get(0).unwrap().as_str();
    token_array.push(get_single_token(tok_str));
  }
  return token_array;
}

#[cfg(test)]
mod tests {
  use super::get_tokens;
  
  #[test]
  fn test_lexer_1() {
    let input_program = r"(n)->(m)";
    println!("{:?}", get_tokens(input_program));
  }

  #[test]
  fn test_lexer_2() {
    let input_program = r"(n:Person)";
    println!("{:?}", get_tokens(input_program));
  }

  #[test]
  fn test_lexer_3() {
    let input_program = r"(n:Person:Swedish)";
    println!("{:?}", get_tokens(input_program));
  }

  #[test]
  fn test_lexer_4() {
    let input_program = r"(n:Person {name: {value}})";
    println!("{:?}", get_tokens(input_program));
  }

  #[test]
  fn test_lexer_5() {
    let input_program = r"(m)<-[:KNOWS]-(n)";
    println!("{:?}", get_tokens(input_program));
  }

  #[test]
  fn test_lexer_6() {
    let input_program = r"(n)-[*]->(m)";
    println!("{:?}", get_tokens(input_program));
  }

  #[test]
  fn test_lexer_7() {
    let input_program = r"(n)-[:KNOWS|LOVES]->(m)";
    println!("{:?}", get_tokens(input_program));
  }

  #[test]
  fn test_lexer_8() {
    let input_program = r"(n)-[:KNOWS]->(m {property: {value}})";
    println!("{:?}", get_tokens(input_program));
  }

  #[test]
  fn test_lexer_9() {
    let input_program = r"(n)-[*1..5]->(m)";
    println!("{:?}", get_tokens(input_program));
  }

  #[test]
  fn test_lexer_10() {
    let input_program = r"(l)->(m)<-(n)";
    println!("{:?}", get_tokens(input_program));
  }
}
