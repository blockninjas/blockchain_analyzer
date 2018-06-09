use super::Hash;
use super::Input;
use super::Output;
use super::ScriptWitness;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
  pub tx_hash: Hash,
  pub witness_hash: Hash,
  pub version: u32,
  pub lock_time: u32,
  pub inputs: Box<[Input]>,
  pub outputs: Box<[Output]>,
  pub script_witnesses: Box<[ScriptWitness]>,
  pub size_in_bytes: u32,
}
