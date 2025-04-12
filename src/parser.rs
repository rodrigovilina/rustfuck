use crate::{node::Node, token::Token};

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
  let maybe_first_token: Option<&Token> = tokens.first();

  match maybe_first_token {
    Some(token) => match token {
      Token::Comma | Token::Dot | Token::Left | Token::Right | Token::Minus | Token::Plus => {
        let (node, rest): (Node, Vec<Token>) = parse_instruction(tokens).unwrap();
        let mut nodes = vec![node];
        nodes.append(&mut parse(rest));
        nodes
      },
      Token::Open => {
        let (node, rest): (Node, Vec<Token>) = parse_loop(tokens).unwrap();
        let mut nodes: Vec<Node> = vec![node];
        nodes.append(&mut parse(rest));
        nodes
      },
      Token::Close => {
        panic!("Unexpected close bracket");
      },
    },
    None => vec![],
  }
}

fn parse_instruction(tokens: Vec<Token>) -> Option<(Node, Vec<Token>)> {
  let head: Token = tokens.first()?.to_owned();
  let tail: Vec<Token> = tokens[1..].to_vec();

  match head {
    Token::Plus | Token::Minus | Token::Right | Token::Left | Token::Dot | Token::Comma => {
      Some((parse_node_from_token(head).unwrap(), tail))
    },
    _ => None,
  }
}

fn parse_loop(tokens: Vec<Token>) -> Option<(Node, Vec<Token>)> {
  let (loop_tokens, rest_tokens) = split_tokens(tokens);
  let loop_node: Node = Node::Loop(parse(loop_tokens));
  Some((loop_node, rest_tokens))
}

fn parse_node_from_token(token: Token) -> Option<Node> {
  match token {
    Token::Dot => Some(Node::Dot),
    Token::Comma => Some(Node::Comma),
    Token::Plus => Some(Node::Plus),
    Token::Minus => Some(Node::Minus),
    Token::Left => Some(Node::Left),
    Token::Right => Some(Node::Right),
    _ => None,
  }
}

fn split_tokens(tokens: Vec<Token>) -> (Vec<Token>, Vec<Token>) {
  let head = tokens.first().unwrap();
  let mut depth: usize = 1;
  let mut loop_tokens: Vec<Token> = vec![];
  let mut close_index: usize = 0;

  if head != &Token::Open {
    panic!("Expected open bracket");
  }

  'lup: for (index, token) in tokens[1..].iter().enumerate() {
    match token {
      Token::Open => depth += 1,
      Token::Close => depth -= 1,
      _ => {},
    }

    if depth == 0 {
      close_index = index;
      break 'lup;
    }

    loop_tokens.push(*token);
  }

  let rest_tokens: Vec<Token> = tokens[close_index + 2..].to_vec();

  (loop_tokens, rest_tokens)
}
