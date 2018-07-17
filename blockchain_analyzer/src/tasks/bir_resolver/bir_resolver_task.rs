use super::{AddressMap, PostgresAddressMap};
use bincode;
use bir;
use config::Config;
use diesel::prelude::*;
use failure::Error;
use rayon::prelude::*;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};
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
    _db_connection: &PgConnection,
  ) -> Result<(), Error> {
    info!("Run BirResolverTask");

    create_dir_all(&config.resolved_bir_file_path)?;

    let mut resolved_bir_files =
      bir::read_bir_files(&config.resolved_bir_file_path)?;
    if let Some(path) = resolved_bir_files.pop() {
      info!("Remove {:?}", path);
      ::std::fs::remove_file(path)?;
    }

    let unresolved_bir_files =
      bir::read_bir_files(&config.unresolved_bir_file_path)?;
    let unresolved_bir_files =
      &unresolved_bir_files[resolved_bir_files.len()..];

    if !unresolved_bir_files.is_empty() {
      unresolved_bir_files
        .par_iter()
        .for_each(|unresolved_bir_file_path| {
          resolve_bir_file(config, unresolved_bir_file_path)
        });
    }

    info!("Finished BirResolverTask");

    Ok(())
  }

  fn get_indexes(&self) -> Vec<Index> {
    vec![]
  }
}

fn resolve_addresses_in_block(
  block: &mut bir::Block,
  db_connection: &PgConnection,
) {
  let mut address_map = PostgresAddressMap::new(&db_connection);

  block
    .transactions
    .iter_mut()
    .flat_map(|transaction| transaction.inputs.iter_mut())
    .for_each(|input| {
      let address = input.address.clone();
      if let bir::Address::Base58Check(ref base58check) = address {
        input.address = bir::Address::Id(address_map.get_id(base58check));
      } else {
        input.address = bir::Address::UnresolvedAddress;
      }
    });

  block
    .transactions
    .iter_mut()
    .flat_map(|transaction| transaction.outputs.iter_mut())
    .for_each(|output| {
      let address = output.address.clone();
      if let bir::Address::Base58Check(ref base58check) = address {
        output.address = bir::Address::Id(address_map.get_id(base58check));
      } else {
        output.address = bir::Address::UnresolvedAddress;
      }
    });
}

fn resolve_bir_file<P>(config: &Config, unresolved_bir_file_path: P)
where
  P: AsRef<Path>,
{
  let db_connection = PgConnection::establish(&config.db_url).unwrap();

  let unresolved_bir_file =
    File::open(unresolved_bir_file_path.as_ref()).unwrap();
  let unresolved_bir_file = BufReader::new(unresolved_bir_file);
  let blocks = bir::BirFileIterator::new(unresolved_bir_file);

  let resolved_bir_file_path = Path::new(&config.resolved_bir_file_path)
    .join(unresolved_bir_file_path.as_ref().file_name().unwrap());

  info!(
    "Resolve {:?} into {:?}",
    unresolved_bir_file_path.as_ref(),
    resolved_bir_file_path
  );

  let resolved_bir_file = File::create(resolved_bir_file_path).unwrap();
  let mut resolved_bir_file = BufWriter::new(resolved_bir_file);

  for mut block in blocks {
    resolve_addresses_in_block(&mut block, &db_connection);
    bincode::serialize_into(&mut resolved_bir_file, &block).unwrap();
  }
}
