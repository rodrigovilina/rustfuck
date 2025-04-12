use crate::{balanced_tokens::BalancedTokens, token::Token};

pub const fn lex_token_from_char(value: char) -> Option<Token> {
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
  BalancedTokens::new(code.chars().filter_map(lex_token_from_char).collect())
}

#[cfg(test)]
mod tests {
  use super::*;

  fn assert_lexed(lexeme: char, expected: Token) {
    let token = lex_token_from_char(lexeme).unwrap();
    assert_eq!(token, expected);
  }

  #[test]
  fn test_lex() {
    assert_lexed('.', Token::Dot);
    assert_lexed(',', Token::Comma);
    assert_lexed('+', Token::Plus);
    assert_lexed('-', Token::Minus);
    assert_lexed('<', Token::Left);
    assert_lexed('>', Token::Right);
    assert_lexed('[', Token::Open);
    assert_lexed(']', Token::Close);
  }
}
