use std::io::{Cursor, Result};
use super::Input;
use byteorder::{LittleEndian, ReadBytesExt};

pub trait ReadInput {
  fn read_input(&mut self) -> Result<Input>;
}

impl<'a> ReadInput for Cursor<&'a [u8]> {
  fn read_input(&mut self) -> Result<Input> {
    let spent_transaction_id = self.read_u64::<LittleEndian>()?;
    let spent_output_index = self.read_u32::<LittleEndian>()?;
    let input = Input::new(spent_transaction_id, spent_output_index);
    Ok(input)
  }
}
