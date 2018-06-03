use super::{Address, Hash};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Input {
  pub sequence_number: u32,
  pub previous_tx_hash: Hash,
  pub previous_tx_output_index: u32,
  pub address: Address,
  pub script: Vec<u8>,
}
