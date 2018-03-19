pub struct NewOutput {
  pub spending_transaction_id: usize,
  pub spending_input_index: usize,
  pub value: u64,
  // TODO How to handle outputs with multiple addresses?
  pub address_id: u32,
}
