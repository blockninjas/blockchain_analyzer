pub struct Output {
  _offset: usize,
}

impl Output {
  pub fn new(offset: usize) -> Output {
    Output { _offset: offset }
  }

  pub fn get_spending_transaction_id(&self) -> usize {
    // TODO Implement
    0
  }

  pub fn get_spending_output_index(&self) -> usize {
    // TODO Implement
    0
  }

  pub fn get_value(&self) -> u64 {
    // TODO Implement
    0
  }

  pub fn get_address_id(&self) -> u32 {
    // TODO Implement
    0
  }
}
