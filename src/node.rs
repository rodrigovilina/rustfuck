#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
  Dot,
  Comma,
  Plus,
  Minus,
  Left,
  Right,
  Loop(Vec<Node>),
}
