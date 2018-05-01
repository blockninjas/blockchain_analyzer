use super::InputOutput;
use read::ReadInputOutput;
use std::io::Cursor;

pub struct InputOutputs<'a> {
  bytes: Cursor<&'a [u8]>,
  number_of_inputs_to_read: u32,
  input_counter: u32,
}

impl<'a> InputOutputs<'a> {
  pub fn new(
    bytes: Cursor<&'a [u8]>,
    number_of_inputs_to_read: u32,
  ) -> InputOutputs<'a> {
    InputOutputs {
      bytes,
      number_of_inputs_to_read,
      input_counter: 0,
    }
  }
}

impl<'a> Iterator for InputOutputs<'a> {
  type Item = InputOutput;

  fn next(&mut self) -> Option<InputOutput> {
    if self.input_counter < self.number_of_inputs_to_read {
      self.input_counter += 1;
      let input_output = self.bytes.read_input_output().unwrap();
      Some(input_output)
    } else {
      None
    }
  }
}
