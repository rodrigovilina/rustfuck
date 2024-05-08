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
