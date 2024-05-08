use crate::node::Node;

#[derive(Debug)]
pub struct Brainfuck<const N: usize> {
  data_pointer: usize,
  data: [u8; N],
  output: Vec<u8>,
  input: Vec<u8>,
}

impl<const N: usize> Brainfuck<N> {
  fn new() -> Self {
    Brainfuck {
      data_pointer: 0,
      data: [0; N],
      output: vec![],
      input: vec![],
    }
  }

  fn run(&mut self, nodes: Vec<Node>) {
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
        }
      }
    }
  }

  fn add(&mut self) {
    self.data[self.data_pointer] = self.data[self.data_pointer].wrapping_add(1);
  }

  fn sub(&mut self) {
    self.data[self.data_pointer] = self.data[self.data_pointer].wrapping_sub(1);
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

  fn dot(&mut self) {
    self.output.append(&mut vec![self.data[self.data_pointer]]);
  }

  fn comma(&mut self) {
    self.data[self.data_pointer] = self.input.pop().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    let mut vm: Brainfuck<10> = Brainfuck::<10>::new();
    vm.data[0] = 255;

    vm.add();
    assert_eq!(vm.data[0], 0);

    vm.add();
    assert_eq!(vm.data[0], 1);
  }

  #[test]
  fn test_sub() {
    let mut vm: Brainfuck<10> = Brainfuck::<10>::new();

    vm.sub();
    assert_eq!(vm.data[0], 255);

    vm.sub();
    assert_eq!(vm.data[0], 254);
  }

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
