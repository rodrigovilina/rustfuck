use crate::{token::Token, BalancedTokens};

pub fn lex(code: &str) -> BalancedTokens {
  BalancedTokens::new(code.chars().enumerate().filter_map(|(u, c)| char_to_token(u, c)).collect())
}

fn char_to_token(_u: usize, _c: char) -> Option<Token> {
  None
}
