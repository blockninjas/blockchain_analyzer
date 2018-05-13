use std::io::{Read, Result};
use super::super::domain::InputOutput;
use byteorder::{LittleEndian, ReadBytesExt};

pub trait ReadInputOutput {
  fn read_input_output(&mut self) -> Result<InputOutput>;
}

impl<R: Read> ReadInputOutput for R {
  fn read_input_output(&mut self) -> Result<InputOutput> {
    let value = self.read_u64::<LittleEndian>().unwrap();
    let address_id = self.read_u64::<LittleEndian>().unwrap();
    let input_output = InputOutput { value, address_id };
    Ok(input_output)
  }
}
