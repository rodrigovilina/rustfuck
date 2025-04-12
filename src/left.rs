use crate::vm::Brainfuck;

pub trait Left {
  fn left(&mut self);
}

impl<const N: usize> Left for Brainfuck<N> {
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
    let mut vm: Brainfuck<2> = Brainfuck::<2>::new();

    vm.left();
    assert_eq!(vm.data_pointer, 1);

    vm.left();
    assert_eq!(vm.data_pointer, 0);
  }
}
