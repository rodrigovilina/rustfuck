use crate::vm::Brainfuck;

pub trait Dot {
  fn dot(&mut self);
}

impl<const N: usize> Dot for Brainfuck<N> {
  fn dot(&mut self) {
    self.output.append(&mut vec![self.data[self.data_pointer]]);
  }
}
