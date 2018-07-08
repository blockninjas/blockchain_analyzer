use super::address_map::{LruCachedAddressMap, PostgresAddressMap};
use super::{InputAddressResolver, OrderedBlocks, State};
use bir;
use blk_file_reader;
use config::Config;
use diesel::{Connection, PgConnection};
use std::path::Path;

/// Constructs the blockchain intermediate representation.
pub fn construct_bir<'a, 'b>(
  config: &Config,
  state: &'a mut State,
) -> impl Iterator<Item = bir::Block> + 'a {
  let current_blk_file = &mut state.current_blk_file;
  let current_blk_file_offset = &mut state.current_blk_file_offset;
  let blocks_to_skip = *current_blk_file_offset;

  let raw_blocks = blk_file_reader::read_blk_files(&config.blk_file_path)
    .unwrap()
    .into_iter()
    .skip(*current_blk_file)
    .flat_map(move |blk_file_path| {
      info!("Construct bir for {}", &blk_file_path);

      let blk_file_name = Path::new(&blk_file_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
      let blk_file_index = blk_file_name[3..8].parse::<usize>().unwrap();

      *current_blk_file = blk_file_index;
      blk_file_reader::read_blocks(&blk_file_path).unwrap()
    })
    .map(move |block| {
      let block = block.unwrap();
      *current_blk_file_offset = block.index_in_blk_file + 1;
      block
    })
    .skip(blocks_to_skip);

  let ordered_blocks = OrderedBlocks::new(
    &mut state.consumed_blocks,
    &mut state.unresolved_blocks,
    &mut state.consumable_blocks,
    raw_blocks,
  );

  info!(
    "Create postgres address map with lru cache of size {}",
    config.address_cache_size
  );

  // TODO Return error instead of panicking.
  // TODO Reuse existing connection.
  let db_connection = PgConnection::establish(&config.db_url).unwrap();
  let address_map = PostgresAddressMap::new(db_connection);
  let address_map =
    LruCachedAddressMap::new(config.address_cache_size, address_map);

  // TODO Reuse existing connection.
  let db_connection = PgConnection::establish(&config.db_url).unwrap();
  let mut input_address_resolver = InputAddressResolver::new(
    address_map,
    db_connection,
    &mut state.utxo_cache,
  );

  // Construct the BIR by chaining the above iterators.
  let next_block_height = &mut state.next_block_height;
  ordered_blocks.map(move |ordered_block| {
    *next_block_height = ordered_block.height as u32 + 1;
    input_address_resolver.resolve_input_addresses(ordered_block)
  })
}
