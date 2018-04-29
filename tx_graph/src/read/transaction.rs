use std::io::Cursor;
use super::Inputs;
use std::mem::size_of;

pub struct Transaction<'a> {
  offset: u64,
  bytes: &'a [u8],
  number_of_inputs: u32,
  number_of_outputs: u32,
}

impl<'a> Transaction<'a> {
  pub fn new(
    offset: u64,
    bytes: &'a [u8],
    number_of_inputs: u32,
    number_of_outputs: u32,
  ) -> Transaction<'a> {
    Transaction {
      offset,
      bytes,
      number_of_inputs,
      number_of_outputs,
    }
  }

  pub fn get_number_of_inputs(&self) -> u32 {
    self.number_of_inputs
  }

  pub fn get_number_of_outputs(&self) -> u32 {
    self.number_of_outputs
  }

  pub fn get_inputs(&self) -> Inputs {
    // TODO Consolidate with `size_of_transaction()`
    let size_of_transaction_header = 2 * size_of::<u32>() as u64;

    let inputs_offset = self.offset + size_of_transaction_header;
    // TODO Fix possibly truncating cast.
    let inputs_offset = inputs_offset as usize;
    let inputs_bytes = &self.bytes[inputs_offset..];
    let cursor = Cursor::new(inputs_bytes);
    Inputs::new(cursor)
  }
}
