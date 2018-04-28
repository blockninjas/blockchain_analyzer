pub struct Input {
  _offset: usize,
}

impl Input {
  pub fn new(offset: usize) -> Input {
    Input { _offset: offset }
  }

  pub fn get_spent_transaction_id(&self) -> usize {
    // TODO Implement
    0
  }

  pub fn get_spent_output_index(&self) -> usize {
    // TODO Implement
    0
  }
}
