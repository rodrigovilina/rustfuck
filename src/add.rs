use crate::vm::Brainfuck;

pub trait Add {
  fn add(&mut self);
}

impl<const N: usize> Add for Brainfuck<N> {
  fn add(&mut self) {
    self.data[self.data_pointer] = self.data[self.data_pointer].wrapping_add(1);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    let mut vm: Brainfuck<2> = Brainfuck::<2>::new();
    vm.data[0] = 255;

    vm.add();
    assert_eq!(vm.data[0], 0);

    vm.add();
    assert_eq!(vm.data[0], 1);
  }
}
