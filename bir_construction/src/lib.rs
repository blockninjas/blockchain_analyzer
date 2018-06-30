//! Blockchain Intermediate Representation (BIR) Construction

extern crate address_map;
extern crate bir;
extern crate blk_file_reader;
extern crate config;
#[macro_use]
extern crate log;
extern crate db_persistence;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

use std::collections::HashMap;

mod bir_construction;
mod input_address_resolver;
mod ordered_blocks;
pub mod state;

pub use bir_construction::construct_bir;
pub use state::State;

use input_address_resolver::InputAddressResolver;
use ordered_blocks::{OrderedBlock, OrderedBlocks};

type TxHash = [u8; 32];

type OutputIndex = u32;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default, Hash, Eq)]
pub struct UtxoId {
  pub tx_hash: TxHash,
  pub output_index: OutputIndex,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Utxo {
  pub address: bir::Address,
  pub value: u64,
}

type UtxoCache = HashMap<UtxoId, Utxo>;

type BlockHash = [u8; 32];

type PreviousBlockHash = [u8; 32];

type BlockHeight = i32;
