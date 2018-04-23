#[derive(Queryable)]
pub struct Input {
  pub id: i64,
  pub sequence_number: i32,
  pub previous_tx_hash: Vec<u8>,
  pub previous_tx_output_index: i32,
  pub script: Vec<u8>,
  pub transaction_id: i64,
}
