use super::{BlockHash, BlockHeight, InputAddressResolver, OrderedBlocks,
            PreviousBlockHash, UtxoCache};
use address_map::{LruCachedAddressMap, PostgresAddressMap};
use bir;
use blk_file_reader;
use config::Config;
use diesel::PgConnection;
use std::collections::HashMap;

/// Constructs the blockchain intermediate representation.
pub fn construct_bir<'a, B>(
  config: &Config,
  db_connection: &'a PgConnection,
  blocks: B,
) -> impl Iterator<Item = bir::Block> + 'a
where
  B: IntoIterator<Item = blk_file_reader::Block> + 'a,
{
  let consumed_blocks: HashMap<BlockHash, BlockHeight> = HashMap::new();

  let unresolved_blocks: HashMap<
    PreviousBlockHash,
    Vec<blk_file_reader::Block>,
  > = HashMap::new();

  let ordered_blocks = OrderedBlocks::new(
    consumed_blocks,
    unresolved_blocks,
    blocks.into_iter(),
  );

  let address_map = PostgresAddressMap::new(&db_connection);
  let address_map =
    LruCachedAddressMap::new(config.address_cache_size, address_map);

  let utxo_cache = UtxoCache::new();

  let mut input_address_resolver =
    InputAddressResolver::new(address_map, utxo_cache);

  // Construct the BIR by chaining the above iterators.
  ordered_blocks
    .map(|ordered_block| ordered_block.block)
    .map(move |block| input_address_resolver.resolve_input_addresses(block))
}
