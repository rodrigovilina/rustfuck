use crate::token::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BalancedTokens {
  pub tokens: Vec<Token>,
}

impl BalancedTokens {
  pub fn new(tokens: Vec<Token>) -> Self {
    let mut brackets: usize = 0;

    for token in &tokens {
      match token {
        Token::Open => brackets += 1,
        Token::Close => brackets -= 1,
        _ => (),
      }
    }

    assert!((brackets == 0), "Unbalanced brackets");

    Self { tokens }
  }
}
