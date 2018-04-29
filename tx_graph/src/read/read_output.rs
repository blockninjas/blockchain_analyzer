use std::io::{Cursor, Result};
use super::Output;
use byteorder::{LittleEndian, ReadBytesExt};

pub trait ReadOutput {
  fn read_output(&mut self) -> Result<Output>;
}

impl<'a> ReadOutput for Cursor<&'a [u8]> {
  fn read_output(&mut self) -> Result<Output> {
    let spending_transaction_id = self.read_u64::<LittleEndian>().unwrap();
    let spending_input_index = self.read_u32::<LittleEndian>().unwrap();
    let value = self.read_u64::<LittleEndian>().unwrap();
    let address_id = self.read_u64::<LittleEndian>().unwrap();
    let output = Output::new(
      spending_transaction_id,
      spending_input_index,
      value,
      address_id,
    );
    Ok(output)
  }
}
