use crate::vm::Brainfuck;

pub trait Comma {
  fn comma(&mut self);
}

impl<const N: usize> Comma for Brainfuck<N> {
  fn comma(&mut self) {
    self.data[self.data_pointer] = self.input.pop().unwrap();
  }
}
