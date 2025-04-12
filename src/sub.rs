use crate::vm::Brainfuck;

pub trait Sub {
  fn sub(&mut self);
}

impl<const N: usize> Sub for Brainfuck<N> {
  fn sub(&mut self) {
    self.data[self.data_pointer] = self.data[self.data_pointer].wrapping_sub(1);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sub() {
    let mut vm: Brainfuck<2> = Brainfuck::<2>::new();

    vm.sub();
    assert_eq!(vm.data[0], 255);

    vm.sub();
    assert_eq!(vm.data[0], 254);
  }
}
