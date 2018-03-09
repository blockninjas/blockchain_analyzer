use schema::inputs;
use blk_file_reader;

#[derive(Insertable)]
#[table_name = "inputs"]
pub struct NewInput {
  pub sequence_number: i32,
  pub previous_tx_hash: Vec<u8>,
  pub previous_tx_output_index: i32,
  pub transaction_id: i32,
}

impl NewInput {
  pub fn new(input: &blk_file_reader::Input, transaction_id: i32) -> NewInput {
    NewInput {
      sequence_number: input.sequence_number as i32,
      previous_tx_hash: input.previous_tx_hash.0.to_vec(),
      previous_tx_output_index: input.previous_tx_output_index as i32,
      transaction_id,
    }
  }
}