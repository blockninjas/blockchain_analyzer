pub struct NewOutput {
  pub spending_transaction_id: u64,
  pub spending_input_index: u32,
  pub value: u64,
  // TODO How to handle outputs with multiple addresses?
  pub address_id: u64,
}
