use super::{Input, ReadInput};
use std::io::Cursor;

pub struct Inputs<'a> {
  bytes: Cursor<&'a [u8]>,
}

impl<'a> Inputs<'a> {
  pub fn new(bytes: Cursor<&'a [u8]>) -> Inputs<'a> {
    Inputs { bytes }
  }
}

impl<'a> Iterator for Inputs<'a> {
  type Item = Input;

  fn next(&mut self) -> Option<Input> {
    if self.bytes.position() < self.bytes.get_ref().len() as u64 {
      let input = self.bytes.read_input().unwrap();
      Some(input)
    } else {
      None
    }
  }
}
