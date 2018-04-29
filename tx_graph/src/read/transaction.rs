use std::io::Cursor;
use super::{Inputs, Outputs};
use super::super::write::NewInput;
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

  pub fn get_outputs(&self) -> Outputs {
    // TODO Consolidate with `size_of_transaction()`
    let size_of_transaction_header = 2 * size_of::<u32>() as u64;
    let size_of_inputs =
      size_of::<NewInput>() as u64 * self.get_number_of_inputs() as u64;

    let outputs_offset =
      self.offset + size_of_transaction_header + size_of_inputs;
    // TODO Fix possibly truncating cast.
    let outputs_offset = outputs_offset as usize;
    let outputs_bytes = &self.bytes[outputs_offset..];
    let cursor = Cursor::new(outputs_bytes);
    Outputs::new(cursor)
  }
}
