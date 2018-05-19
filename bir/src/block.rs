use super::{Hash, Transaction};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Block {
  pub hash: Hash,
  pub height: u32,
  pub version: u32,
  pub previous_block_hash: Hash,
  pub merkle_root: Hash,
  pub creation_time: u32,
  pub bits: u32,
  pub nonce: u32,
  pub transactions: Vec<Transaction>,
}
