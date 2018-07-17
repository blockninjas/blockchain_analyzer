use super::{BlockHash, BlockHeight, PreviousBlockHash, UtxoCache};
use bincode;
use blk_file_reader;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct State {
  pub current_blk_file: usize,
  pub current_blk_file_offset: usize,
  pub next_block_height: u32,
  pub consumed_blocks: HashMap<BlockHash, BlockHeight>,
  pub unresolved_blocks:
    HashMap<PreviousBlockHash, Vec<blk_file_reader::Block>>,
  pub consumable_blocks: VecDeque<blk_file_reader::Block>,
  pub utxo_cache: UtxoCache,
}

pub fn save_state<P>(state: State, path: P)
where
  P: AsRef<Path>,
{
  // TODO Return error instead of panicking.
  let state_file = File::create(path).unwrap();
  let mut state_file = BufWriter::new(state_file);
  // TODO Return error instead of panicking.
  bincode::serialize_into(&mut state_file, &state).unwrap();
}

pub fn load_state<P>(path: P) -> State
where
  P: AsRef<Path>,
{
  if path.as_ref().exists() {
    // TODO Return error instead of panicking.
    let state_file = File::open(path).unwrap();
    let mut state_file = BufReader::new(state_file);
    // TODO Return error instead of panicking.
    bincode::deserialize_from(&mut state_file).unwrap()
  } else {
    initial_state()
  }
}

pub fn initial_state() -> State {
  let mut consumed_blocks = HashMap::new();
  consumed_blocks.insert([0u8; 32], -1);

  State {
    current_blk_file: 0,
    current_blk_file_offset: 0,
    next_block_height: 0,
    consumed_blocks,
    unresolved_blocks: HashMap::new(),
    consumable_blocks: VecDeque::new(),
    utxo_cache: UtxoCache::new(),
  }
}
