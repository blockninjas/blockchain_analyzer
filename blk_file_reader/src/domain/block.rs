use super::Hash;
use super::Transaction;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
  pub hash: Hash,
  pub version: u32,
  pub previous_block_hash: Hash,
  pub merkle_root: Hash,
  pub creation_time: u32,
  pub bits: u32,
  pub nonce: u32,
  pub transactions: Box<[Transaction]>,
  pub index_in_blk_file: usize,
}
