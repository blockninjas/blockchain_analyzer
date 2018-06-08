use super::Hash;
use super::Input;
use super::Output;
use super::Witness;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
  pub tx_hash: Hash,
  pub witness_hash: Hash,
  pub version: u32,
  pub lock_time: u32,
  pub inputs: Box<[Input]>,
  pub outputs: Box<[Output]>,
  pub witnesses: Box<[Witness]>,
  pub size_in_bytes: u32,
}
