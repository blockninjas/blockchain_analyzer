use std::io::{Cursor, Result};
use super::super::domain::TransactionHeader;
use byteorder::{LittleEndian, ReadBytesExt};

pub trait ReadTransactionHeader {
  fn read_transaction_header(&mut self) -> Result<TransactionHeader>;
}

impl<'a> ReadTransactionHeader for Cursor<&'a [u8]> {
  fn read_transaction_header(&mut self) -> Result<TransactionHeader> {
    let number_of_inputs = self.read_u32::<LittleEndian>()?;
    let number_of_outputs = self.read_u32::<LittleEndian>()?;
    let transaction_header = TransactionHeader {
      number_of_inputs,
      number_of_outputs,
    };
    Ok(transaction_header)
  }
}
