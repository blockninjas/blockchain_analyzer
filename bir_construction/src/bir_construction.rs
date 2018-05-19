use super::{BlockHash, BlockHeight, InputAddressResolver, OrderedBlocks,
            PreviousBlockHash, UtxoCache};
use address_map::{LruCachedAddressMap, RedisAddressMap};
use bir;
use blk_file_reader;
use config::Config;
use redis;
use std::collections::HashMap;

/// Constructs the blockchain intermedate representation from the given blocks.
// TODO Provide possibility to capture BIR construction state.
pub fn construct_bir<B>(
  config: &Config,
  blocks: B,
) -> impl Iterator<Item = bir::Block>
where
  B: Iterator<Item = blk_file_reader::Block>,
{
  let consumed_blocks: HashMap<BlockHash, BlockHeight> = HashMap::new();

  let unresolved_blocks: HashMap<
    PreviousBlockHash,
    Vec<blk_file_reader::Block>,
  > = HashMap::new();

  let ordered_blocks =
    OrderedBlocks::new(consumed_blocks, unresolved_blocks, blocks);

  let client = redis::Client::open(&config.redis_url[..]).unwrap();
  let connection = client.get_connection().unwrap();
  let address_map = RedisAddressMap::new(connection);

  let address_map =
    LruCachedAddressMap::new(config.address_cache_size, address_map);

  // TODO Make configurable.
  let utxo_cache = UtxoCache::new();

  let mut input_address_resolver =
    InputAddressResolver::new(address_map, utxo_cache);

  ordered_blocks
    .map(|ordered_block| ordered_block.block)
    .map(move |block| input_address_resolver.resolve_input_addresses(block))
}
