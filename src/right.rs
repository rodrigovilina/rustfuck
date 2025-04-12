use crate::vm::Brainfuck;

pub trait Right {
  fn right(&mut self);
}

impl<const N: usize> Right for Brainfuck<N> {
  fn right(&mut self) {
    if self.data_pointer == N - 1 {
      self.data_pointer = 0;
    } else {
      self.data_pointer += 1;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rigth() {
    let mut vm: Brainfuck<2> = Brainfuck::<2>::new();
    vm.data_pointer = 1;

    vm.right();
    assert_eq!(vm.data_pointer, 0);

    vm.right();
    assert_eq!(vm.data_pointer, 1);
  }
}
