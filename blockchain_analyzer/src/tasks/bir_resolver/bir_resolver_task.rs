use super::{
  AddressMap, InMemoryAddressMap, LruCachedAddressMap, PostgresAddressMap,
};
use bincode;
use bir;
use config::Config;
use diesel::prelude::*;
use failure::Error;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::collections::HashMap;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::result::Result;
use task_manager::{Index, Task};

pub struct BirResolverTask {}

impl BirResolverTask {
  pub fn new() -> BirResolverTask {
    BirResolverTask {}
  }
}

impl Task for BirResolverTask {
  fn run(
    &self,
    config: &Config,
    db_connection_pool: &Pool<ConnectionManager<PgConnection>>,
  ) -> Result<(), Error> {
    info!("Run BirResolverTask");

    create_dir_all(&config.resolved_bir_file_path)?;

    let resolved_bir_files =
      bir::read_bir_files(&config.resolved_bir_file_path)?;

    let db_connection = db_connection_pool.get()?;

    if let Some(path) = resolved_bir_files.last() {
      continue_to_resolve_bir_file(config, &db_connection, path);
    }

    let unresolved_bir_files =
      bir::read_bir_files(&config.unresolved_bir_file_path)?;
    let unresolved_bir_files =
      &unresolved_bir_files[resolved_bir_files.len()..];

    if !unresolved_bir_files.is_empty() {
      let addresses = super::in_memory_address_map::load_all_addresses(
        config,
        &db_connection,
      )?;
      let mut address_map = InMemoryAddressMap::new(addresses);

      for unresolved_bir_file in unresolved_bir_files {
        resolve_new_bir_file(&mut address_map, config, unresolved_bir_file);
      }
    }

    info!("Finished BirResolverTask");

    Ok(())
  }

  fn get_indexes(&self) -> Vec<Index> {
    vec![]
  }
}

fn continue_to_resolve_bir_file<P>(
  config: &Config,
  db_connection: &PgConnection,
  resolved_bir_file_path: P,
) where
  P: AsRef<Path>,
{
  let resolved_blocks = read_blocks_from_bir_file(&resolved_bir_file_path);
  let number_of_resolved_blocks = resolved_blocks.count();

  let unresolved_bir_file_path = Path::new(&config.unresolved_bir_file_path)
    .join(resolved_bir_file_path.as_ref().file_name().unwrap());
  let unresolved_blocks = read_blocks_from_bir_file(&unresolved_bir_file_path);
  let unresolved_blocks = unresolved_blocks.skip(number_of_resolved_blocks);

  info!(
    "Continue to resolve {:?} into {:?}, skipping {} blocks",
    unresolved_bir_file_path.as_path(),
    resolved_bir_file_path.as_ref(),
    number_of_resolved_blocks
  );

  let resolved_bir_file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(&resolved_bir_file_path)
    .unwrap();
  let mut resolved_bir_file = BufWriter::new(resolved_bir_file);

  let mut address_map = PostgresAddressMap::new(db_connection);

  resolve_blocks_into_file(
    &mut address_map,
    unresolved_blocks,
    &mut resolved_bir_file,
  );
}

fn resolve_new_bir_files<P>(config: &Config, unresolved_bir_files: &[P])
where
  P: AsRef<Path>,
{
  info!(
    "Resolve {:?} to {:?}",
    unresolved_bir_files.first().unwrap().as_ref(),
    unresolved_bir_files.last().unwrap().as_ref()
  );

  let db_connection = PgConnection::establish(&config.db_url).unwrap();
  let address_map = PostgresAddressMap::new(&db_connection);
  let mut address_map =
    LruCachedAddressMap::new(config.address_cache_size, address_map);

  for path in unresolved_bir_files.iter() {
    resolve_new_bir_file(&mut address_map, config, path);
  }

  info!(
    "Resolve {:?} to {:?} with {} cache hits and {} cache misses",
    unresolved_bir_files.first().unwrap().as_ref(),
    unresolved_bir_files.last().unwrap().as_ref(),
    address_map.get_cache_hits(),
    address_map.get_cache_misses()
  );
}

fn resolve_new_bir_file<P>(
  address_map: &mut dyn AddressMap,
  config: &Config,
  unresolved_bir_file_path: P,
) where
  P: AsRef<Path>,
{
  let unresolved_blocks = read_blocks_from_bir_file(&unresolved_bir_file_path);
  let resolved_bir_file_path = Path::new(&config.resolved_bir_file_path)
    .join(unresolved_bir_file_path.as_ref().file_name().unwrap());

  info!(
    "Resolve {:?} into {:?}",
    unresolved_bir_file_path.as_ref(),
    resolved_bir_file_path
  );

  let resolved_bir_file = File::create(resolved_bir_file_path).unwrap();
  let mut resolved_bir_file = BufWriter::new(resolved_bir_file);

  resolve_blocks_into_file(
    address_map,
    unresolved_blocks,
    &mut resolved_bir_file,
  );
}

fn read_blocks_from_bir_file<P>(path: P) -> impl Iterator<Item = bir::Block>
where
  P: AsRef<Path>,
{
  let bir_file = File::open(path).unwrap();
  let bir_file = BufReader::new(bir_file);
  bir::BirFileIterator::new(bir_file)
}

fn resolve_blocks_into_file<U>(
  address_map: &mut dyn AddressMap,
  unresolved_blocks: U,
  mut resolved_bir_file: &mut dyn Write,
) where
  U: IntoIterator<Item = bir::Block>,
{
  for mut block in unresolved_blocks {
    resolve_addresses_in_block(&mut block, address_map);
    bincode::serialize_into(&mut resolved_bir_file, &block).unwrap();
  }
}

fn resolve_addresses_in_block(
  block: &mut bir::Block,
  address_map: &mut dyn AddressMap,
) {
  let addresses = get_base58check_addresses_in_block(block);
  let address_ids = address_map.get_ids(&addresses);

  // Resolve input addresses
  block
    .transactions
    .iter_mut()
    .flat_map(|transaction| transaction.inputs.iter_mut())
    .for_each(|input| {
      input.address = resolve_address(&input.address, &address_ids);
    });

  // Resolve output addresses
  block
    .transactions
    .iter_mut()
    .flat_map(|transaction| transaction.outputs.iter_mut())
    .for_each(|output| {
      output.address = resolve_address(&output.address, &address_ids);
    });
}

fn get_base58check_addresses_in_block(block: &bir::Block) -> Vec<String> {
  let input_addresses: Vec<String> = block
    .transactions
    .iter()
    .flat_map(|transaction| transaction.inputs.iter())
    .filter_map(|input| {
      if let bir::Address::Base58Check(ref base58check) = input.address {
        Some(base58check.clone())
      } else {
        None
      }
    })
    .collect();

  let mut output_addresses: Vec<String> = block
    .transactions
    .iter()
    .flat_map(|transaction| transaction.outputs.iter())
    .filter_map(|output| {
      if let bir::Address::Base58Check(ref base58check) = output.address {
        Some(base58check.clone())
      } else {
        None
      }
    })
    .collect();

  let mut addresses = input_addresses;
  addresses.append(&mut output_addresses);
  addresses
}

fn resolve_address(
  address: &bir::Address,
  address_ids: &HashMap<String, u64>,
) -> bir::Address {
  if let bir::Address::Base58Check(base58check) = address {
    if let Some(&address_id) = address_ids.get(base58check) {
      return bir::Address::Id(address_id);
    }
  }
  bir::Address::UnresolvedAddress
}
