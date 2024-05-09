use crate::token::{BalancedTokens, Token};

pub fn lex_token_from_char(value: char) -> Option<Token> {
  match value {
    '.' => Some(Token::Dot),
    ',' => Some(Token::Comma),
    '+' => Some(Token::Plus),
    '-' => Some(Token::Minus),
    '<' => Some(Token::Left),
    '>' => Some(Token::Right),
    '[' => Some(Token::Open),
    ']' => Some(Token::Close),
    _ => None,
  }
}

pub fn lex(code: &str) -> BalancedTokens {
  BalancedTokens::new(
    code
      .chars()
      .enumerate()
      .filter_map(|(_, c)| lex_token_from_char(c))
      .collect(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn lex_dot() {
    let token = lex_token_from_char('.').unwrap();
    assert_eq!(token, Token::Dot);
  }

  #[test]
  fn lex_comma() {
    let token = lex_token_from_char(',').unwrap();
    assert_eq!(token, Token::Comma);
  }

  #[test]
  fn lex_plus() {
    let token = lex_token_from_char('+').unwrap();
    assert_eq!(token, Token::Plus);
  }

  #[test]
  fn lex_minus() {
    let token = lex_token_from_char('-').unwrap();
    assert_eq!(token, Token::Minus);
  }

  #[test]
  fn lex_left() {
    let token = lex_token_from_char('<').unwrap();
    assert_eq!(token, Token::Left);
  }

  #[test]
  fn lex_right() {
    let token = lex_token_from_char('>').unwrap();
    assert_eq!(token, Token::Right);
  }

  #[test]
  fn lex_open() {
    let token = lex_token_from_char('[').unwrap();
    assert_eq!(token, Token::Open);
  }

  #[test]
  fn lex_close() {
    let token = lex_token_from_char(']').unwrap();
    assert_eq!(token, Token::Close);
  }
}
