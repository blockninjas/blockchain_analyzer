use super::InputOutput;
use read::ReadInputOutput;
use std::io::Cursor;

pub struct InputOutputs<'a> {
  bytes: Cursor<&'a [u8]>,
}

impl<'a> InputOutputs<'a> {
  pub fn new(bytes: Cursor<&'a [u8]>) -> InputOutputs<'a> {
    InputOutputs { bytes }
  }
}

impl<'a> Iterator for InputOutputs<'a> {
  type Item = InputOutput;

  fn next(&mut self) -> Option<InputOutput> {
    if self.bytes.position() < self.bytes.get_ref().len() as u64 {
      let input_output = self.bytes.read_input_output().unwrap();
      Some(input_output)
    } else {
      None
    }
  }
}
