pub struct Transaction<'a> {
  _offset: u64,
  _bytes: &'a [u8],
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
      _offset: offset,
      _bytes: bytes,
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
}
