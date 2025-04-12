use crate::{add::Add, comma::Comma, dot::Dot, node::Node, sub::Sub};

#[derive(Debug)]
pub struct Brainfuck<const N: usize> {
  pub data_pointer: usize,
  pub data: [u8; N],
  pub output: Vec<u8>,
  pub input: Vec<u8>,
}

impl<const N: usize> Brainfuck<N> {
  pub fn new() -> Self {
    Brainfuck {
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
      println!("Output: {}", string)
    }
  }

  pub fn run(&mut self, nodes: Vec<Node>) {
    for node in nodes {
      match node {
        Node::Right => self.right(),
        Node::Left => self.left(),
        Node::Dot => self.dot(),
        Node::Comma => self.comma(),
        Node::Plus => self.add(),
        Node::Minus => self.sub(),
        Node::Loop(nodes) => {
          while self.data[self.data_pointer] != 0 {
            self.run(nodes.clone());
          }
        },
      }
    }
  }

  fn right(&mut self) {
    match self.data_pointer {
      a if a == N - 1 => self.data_pointer = 0,
      _ => self.data_pointer += 1,
    }
  }

  fn left(&mut self) {
    match self.data_pointer {
      0 => self.data_pointer = N - 1,
      _ => self.data_pointer -= 1,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_left() {
    let mut vm: Brainfuck<10> = Brainfuck::<10>::new();

    vm.left();
    assert_eq!(vm.data_pointer, 9);

    vm.left();
    assert_eq!(vm.data_pointer, 8);
  }

  #[test]
  fn test_rigth() {
    let mut vm: Brainfuck<10> = Brainfuck::<10>::new();
    vm.data_pointer = 9;

    vm.right();
    assert_eq!(vm.data_pointer, 0);

    vm.right();
    assert_eq!(vm.data_pointer, 1);
  }
}
