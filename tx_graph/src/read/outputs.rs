use super::{Output, ReadOutput};
use std::io::Cursor;

pub struct Outputs<'a> {
  bytes: Cursor<&'a [u8]>,
}

impl<'a> Outputs<'a> {
  pub fn new(bytes: Cursor<&'a [u8]>) -> Outputs<'a> {
    Outputs { bytes }
  }
}

impl<'a> Iterator for Outputs<'a> {
  type Item = Output;

  fn next(&mut self) -> Option<Output> {
    if self.bytes.position() < self.bytes.get_ref().len() as u64 {
      let output = self.bytes.read_output().unwrap();
      Some(output)
    } else {
      None
    }
  }
}
