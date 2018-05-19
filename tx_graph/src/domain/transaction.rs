use std::io::Cursor;
use super::{InputOutputs, TransactionHeader};
use domain::memory_layout::{size_of_input_output, size_of_transaction_header};

pub struct Transaction<'a> {
  offset: u64,
  bytes: &'a [u8],
  header: TransactionHeader,
}

impl<'a> Transaction<'a> {
  pub fn new(
    offset: u64,
    bytes: &'a [u8],
    header: TransactionHeader,
  ) -> Transaction<'a> {
    Transaction {
      offset,
      bytes,
      header,
    }
  }

  pub fn get_offset(&self) -> u64 {
    self.offset
  }

  pub fn get_number_of_inputs(&self) -> u32 {
    self.header.number_of_inputs
  }

  pub fn get_number_of_outputs(&self) -> u32 {
    self.header.number_of_outputs
  }

  pub fn get_inputs(&self) -> InputOutputs {
    // TODO Fix possibly truncating cast
    let offset_of_inputs = self.get_offset_of_inputs() as usize;
    let inputs_as_bytes = &self.bytes[offset_of_inputs..];
    let cursor = Cursor::new(inputs_as_bytes);
    InputOutputs::new(cursor, self.get_number_of_inputs())
  }

  pub fn get_outputs(&self) -> InputOutputs {
    // TODO Fix possibly truncating cast.
    let offset_of_outputs = self.get_offset_of_outputs() as usize;
    let outputs_as_bytes = &self.bytes[offset_of_outputs..];
    let cursor = Cursor::new(outputs_as_bytes);
    InputOutputs::new(cursor, self.get_number_of_outputs())
  }
}

pub trait TransactionMemoryLayout {
  fn get_offset_of_inputs(&self) -> u64;

  fn get_offset_of_outputs(&self) -> u64;

  fn get_size_of_inputs(&self) -> u64;

  fn get_size_of_outputs(&self) -> u64;

  fn get_size(&self) -> u64;
}

impl<'a> TransactionMemoryLayout for Transaction<'a> {
  fn get_offset_of_inputs(&self) -> u64 {
    self.offset + size_of_transaction_header() as u64
  }

  fn get_offset_of_outputs(&self) -> u64 {
    self.get_offset_of_inputs() + self.get_size_of_inputs()
  }

  fn get_size_of_inputs(&self) -> u64 {
    self.get_number_of_inputs() as u64 * size_of_input_output() as u64
  }

  fn get_size_of_outputs(&self) -> u64 {
    self.get_number_of_outputs() as u64 * size_of_input_output() as u64
  }

  fn get_size(&self) -> u64 {
    size_of_transaction_header() as u64 + self.get_size_of_inputs()
      + self.get_size_of_outputs()
  }
}