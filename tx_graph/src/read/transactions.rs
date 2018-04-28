use super::{ReadTransaction, Transaction};
use write::{NewInput, NewOutput};
use std::io::Cursor;
use std::mem::size_of;

pub struct Transactions<'a> {
  offset: u64,
  bytes: &'a [u8],
}

impl<'a> Transactions<'a> {
  pub fn new<B: 'a + AsRef<[u8]>>(bytes: &'a B) -> Transactions<'a> {
    Transactions {
      offset: 0,
      bytes: bytes.as_ref(),
    }
  }
}

pub fn size_of_transaction(transaction: &Transaction) -> u64 {
  let size_of_number_of_inputs = size_of::<u32>() as u64;
  let size_of_number_of_outputs = size_of::<u32>() as u64;
  let size_of_inputs =
    size_of::<NewInput>() as u64 * transaction.get_number_of_inputs() as u64;
  let size_of_outputs =
    size_of::<NewOutput>() as u64 * transaction.get_number_of_outputs() as u64;

  size_of_number_of_inputs + size_of_number_of_outputs + size_of_inputs
    + size_of_outputs
}

impl<'a> Iterator for Transactions<'a> {
  type Item = Transaction<'a>;

  fn next(&mut self) -> Option<Transaction<'a>> {
    if self.offset < self.bytes.len() as u64 {
      let mut cursor = Cursor::new(self.bytes);
      let transaction = cursor.read_transaction().unwrap();
      self.offset += size_of_transaction(&transaction);
      Some(transaction)
    } else {
      None
    }
  }
}
