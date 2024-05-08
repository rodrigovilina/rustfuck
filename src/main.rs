mod node;
mod token;
mod vm;

use token::Token;

fn main() {}

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
}
