use crate::{add::Add, comma::Comma, dot::Dot, left::Left, node::Node, right::Right, sub::Sub};

#[derive(Debug)]
pub struct Brainfuck<const N: usize> {
  pub data_pointer: usize,
  pub data: [u8; N],
  pub output: Vec<u8>,
  pub input: Vec<u8>,
}

impl<const N: usize> Brainfuck<N> {
  pub const fn new() -> Self {
    Self {
      data_pointer: 0,
      data: [0; N],
      output: vec![],
      input: vec![],
    }
  }

  pub fn debug(&self) {
    println!("Data Pointer: {}", self.data_pointer);
    println!("Memory:");
    for (i, &x) in self.data.iter().enumerate() {
      if x != 0 {
        println!("  Index: {}, Value: {} {}", i, x, x as char);
      }
    }
    println!("Input: {:?}", self.input);
    println!("Output: {:?}", self.output);

    if let Ok(string) = String::from_utf8(self.output.clone()) {
      println!("Output: {string}");
    }
  }

  pub fn run(&mut self, nodes: &Vec<Node>) {
    nodes.iter().for_each(|node| self.run_node(node));
  }

  fn run_node(&mut self, node: &Node) {
    match node {
      Node::Right => self.right(),
      Node::Left => self.left(),
      Node::Dot => self.dot(),
      Node::Comma => self.comma(),
      Node::Plus => self.add(),
      Node::Minus => self.sub(),
      Node::Loop(nodes) => self.run_loop(nodes),
    }
  }

  fn run_loop(&mut self, nodes: &Vec<Node>) {
    while self.data[self.data_pointer] != 0 {
      self.run(nodes);
    }
  }
}
