use super::{Hash, Input, Output};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Transaction {
  pub tx_hash: Hash,
  pub version: u32,
  pub lock_time: u32,
  pub inputs: Vec<Input>,
  pub outputs: Vec<Output>,
}
