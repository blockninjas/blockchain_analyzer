use std::io::{Cursor, Result};
use super::Transaction;
use byteorder::{LittleEndian, ReadBytesExt};

pub trait ReadTransaction<'a> {
  fn read_transaction(&mut self) -> Result<Transaction<'a>>;
}

impl<'a> ReadTransaction<'a> for Cursor<&'a [u8]> {
  fn read_transaction(&mut self) -> Result<Transaction<'a>> {
    let number_of_inputs = self.read_u32::<LittleEndian>()?;
    let number_of_outputs = self.read_u32::<LittleEndian>()?;
    let transaction =
      Transaction::new(0, self.get_ref(), number_of_inputs, number_of_outputs);
    Ok(transaction)
  }
}
