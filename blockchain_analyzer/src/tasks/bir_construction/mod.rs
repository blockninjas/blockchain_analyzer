use bir;
use std::collections::HashMap;

mod bir_construction;
mod bir_construction_task;
mod input_address_resolver;
mod ordered_blocks;
pub mod state;

pub use self::bir_construction::construct_bir;
pub use self::bir_construction_task::BirConstructionTask;
pub use self::state::State;

use self::input_address_resolver::InputAddressResolver;
use self::ordered_blocks::{OrderedBlock, OrderedBlocks};

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
