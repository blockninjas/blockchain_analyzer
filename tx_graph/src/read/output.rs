pub struct Output {
  spending_transaction_id: u64,
  spending_input_index: u32,
  value: u64,
  address_id: u64,
}

impl Output {
  pub fn new(
    spending_transaction_id: u64,
    spending_input_index: u32,
    value: u64,
    address_id: u64,
  ) -> Output {
    Output {
      spending_transaction_id,
      spending_input_index,
      value,
      address_id,
    }
  }

  pub fn get_spending_transaction_id(&self) -> u64 {
    self.spending_transaction_id
  }

  pub fn get_spending_input_index(&self) -> u32 {
    self.spending_input_index
  }

  pub fn get_value(&self) -> u64 {
    self.value
  }

  pub fn get_address_id(&self) -> u64 {
    self.address_id
  }
}
