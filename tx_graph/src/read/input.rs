pub struct Input {
  spent_transaction_id: u64,
  spent_output_index: u32,
}

impl Input {
  pub fn new(spent_transaction_id: u64, spent_output_index: u32) -> Input {
    Input {
      spent_transaction_id,
      spent_output_index,
    }
  }

  pub fn get_spent_transaction_id(&self) -> u64 {
    self.spent_transaction_id
  }

  pub fn get_spent_output_index(&self) -> u32 {
    self.spent_output_index
  }
}
