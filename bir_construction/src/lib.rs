//! Blockchain Intermediate Representation (BIR) Construction

extern crate address_map;
extern crate bir;
extern crate blk_file_reader;
extern crate config;
#[macro_use]
extern crate log;
extern crate dotenv;
extern crate redis;

use std::collections::HashMap;

mod bir_construction;
mod input_address_resolver;
mod ordered_blocks;

pub use bir_construction::construct_bir;

use input_address_resolver::InputAddressResolver;
use ordered_blocks::OrderedBlocks;
type TxHash = [u8; 32];
type OutputIndex = u32;
#[derive(Hash, PartialEq, Eq)]
pub struct UtxoId {
  pub tx_hash: TxHash,
  pub output_index: OutputIndex,
}
type AddressId = u64;
type UtxoCache = HashMap<UtxoId, AddressId>;
type BlockHash = [u8; 32];
type PreviousBlockHash = [u8; 32];
type BlockHeight = i32;
