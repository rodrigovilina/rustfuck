use crate::node::Node;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
  Dot,
  Comma,
  Plus,
  Minus,
  Left,
  Right,
  Open,
  Close,
}

impl Token {
  pub fn try_from_char(value: char) -> Option<Self> {
    match value {
      '.' => Some(Self::Dot),
      ',' => Some(Self::Comma),
      '+' => Some(Self::Plus),
      '-' => Some(Self::Minus),
      '<' => Some(Self::Left),
      '>' => Some(Self::Right),
      '[' => Some(Self::Open),
      ']' => Some(Self::Close),
      _ => None,
    }
  }

  pub fn try_to_node(&self) -> Option<Node> {
    match self {
      Token::Dot => Some(Node::Dot),
      Token::Comma => Some(Node::Comma),
      Token::Plus => Some(Node::Plus),
      Token::Minus => Some(Node::Minus),
      Token::Left => Some(Node::Left),
      Token::Right => Some(Node::Right),
      _ => None,
    }
  }
}

pub struct BalancedTokens {
  pub tokens: Vec<Token>,
}

impl BalancedTokens {
  pub fn new(tokens: Vec<Token>) -> Self {
    let mut brackets: usize = 0;

    for token in tokens.iter() {
      match token {
        Token::Open => brackets += 1,
        Token::Close => brackets -= 1,
        _ => (),
      }
    }

    if brackets != 0 {
      panic!("Unbalanced brackets");
    }

    Self { tokens }
  }

  pub fn lex(code: &str) -> Self {
    Self::new(
      code
        .chars()
        .enumerate()
        .filter_map(|(_, c)| Token::try_from_char(c))
        .collect(),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn lex_dot() {
    let token = Token::try_from_char('.').unwrap();
    assert_eq!(token, Token::Dot);
  }

  #[test]
  fn lex_comma() {
    let token = Token::try_from_char(',').unwrap();
    assert_eq!(token, Token::Comma);
  }

  #[test]
  fn lex_plus() {
    let token = Token::try_from_char('+').unwrap();
    assert_eq!(token, Token::Plus);
  }

  #[test]
  fn lex_minus() {
    let token = Token::try_from_char('-').unwrap();
    assert_eq!(token, Token::Minus);
  }

  #[test]
  fn lex_left() {
    let token = Token::try_from_char('<').unwrap();
    assert_eq!(token, Token::Left);
  }

  #[test]
  fn lex_right() {
    let token = Token::try_from_char('>').unwrap();
    assert_eq!(token, Token::Right);
  }

  #[test]
  fn lex_open() {
    let token = Token::try_from_char('[').unwrap();
    assert_eq!(token, Token::Open);
  }

  #[test]
  fn lex_close() {
    let token = Token::try_from_char(']').unwrap();
    assert_eq!(token, Token::Close);
  }
}
