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
  Space,
  NewLine,
}

impl Token {
  pub const fn to_node(&self) -> Option<Node> {
    match self {
      Self::Dot => Some(Node::Dot),
      Self::Comma => Some(Node::Comma),
      Self::Plus => Some(Node::Plus),
      Self::Minus => Some(Node::Minus),
      Self::Left => Some(Node::Left),
      Self::Right => Some(Node::Right),
      Self::Space => Some(Node::Space),
      Self::NewLine => Some(Node::NewLine),
      _ => None,
    }
  }
}

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
