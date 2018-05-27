use super::Hash;
use super::Input;
use super::Output;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
  pub tx_hash: Hash,
  pub version: u32,
  pub lock_time: u32,
  pub inputs: Box<[Input]>,
  pub outputs: Box<[Output]>,
  pub block_height: u64,
}
