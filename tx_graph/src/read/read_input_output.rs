use std::io::{Cursor, Result};
use super::super::domain::InputOutput;
use byteorder::{LittleEndian, ReadBytesExt};

pub trait ReadInputOutput {
  fn read_input_output(&mut self) -> Result<InputOutput>;
}

impl<'a> ReadInputOutput for Cursor<&'a [u8]> {
  fn read_input_output(&mut self) -> Result<InputOutput> {
    let value = self.read_u64::<LittleEndian>().unwrap();
    let address_id = self.read_u64::<LittleEndian>().unwrap();
    let input_output = InputOutput { value, address_id };
    Ok(input_output)
  }
}
