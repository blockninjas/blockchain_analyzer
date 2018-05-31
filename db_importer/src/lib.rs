extern crate blk_file_reader;
extern crate db_persistence;
extern crate diesel;
#[macro_use]
extern crate log;
extern crate config;
extern crate rayon;

mod address_deduplicator;
mod blkfileimporter;

pub use blkfileimporter::BlkFileImporter;

use blk_file_reader::{read_blk_files, read_blocks};
use config::Config;
use db_persistence::repository::{BlkFileRepository, BlockRepository};
use diesel::prelude::*;
use rayon::prelude::*;
use std::collections::HashSet;

/// Imports the blk files at `path` into the database at `database_url`.
pub fn import_blk_files(config: &Config) -> std::io::Result<()> {
  // TODO Return error instead of panicking.
  let db_connection = PgConnection::establish(&config.db_url).unwrap();

  // Get the blk files that have already been imported by previous runs.
  let blk_file_repository = BlkFileRepository::new(&db_connection);
  let imported_blk_file_names: HashSet<_> = blk_file_repository
    .read_all_names()
    .into_iter()
    .collect();

  let blk_files = read_blk_files(&config.blk_file_path)?;

  // Do not import the latest 2 blk files to be able to ignore blockchain
  // reorganizations.
  // TODO Make this configurable.
  let number_of_files_to_skip_at_end = 2;
  let number_files_to_import = blk_files.len() - number_of_files_to_skip_at_end;

  // TODO Make number of threads configurable.
  // TODO Handle failing threads.
  blk_files
    .par_iter()
    .take(number_files_to_import)
    .filter(|&blk_file| {
      !imported_blk_file_names.contains(&get_blk_file_name(blk_file))
    })
    .for_each(|blk_file| import_blk_file(blk_file, &config.db_url));

  // Finally, calculate the height for all blocks.
  // TODO Do not always recalculate for the whole blockchain.
  // TODO Execute this within a transaction?
  let block_repository = BlockRepository::new(&db_connection);
  block_repository.calculate_block_height();

  // TODO Execute this within a transaction.
  address_deduplicator::deduplicate_output_addresses(&db_connection);

  Ok(())
}

fn get_blk_file_name(blk_file_path: &str) -> String {
  String::from(
    std::path::Path::new(blk_file_path)
      .file_name()
      .unwrap()
      .to_str()
      .unwrap(),
  )
}

/// Imports a blk file into the database at `database_url`.
fn import_blk_file(blk_file_path: &str, database_url: &str) {
  info!("Import {}", blk_file_path);

  // TODO Return error instead of panicking.
  let db_connection = PgConnection::establish(database_url).unwrap();
  let transaction_result = db_connection
    .transaction::<(), diesel::result::Error, _>(|| {
      // TODO Return error instead of panicking.
      let blocks = read_blocks(blk_file_path).unwrap();
      let blk_file_importer = BlkFileImporter::new(&db_connection);
      blk_file_importer.import(blk_file_path, blocks)
    });

  match transaction_result {
    Ok(_) => {
      info!("Finished import of {}", blk_file_path);
    }
    Err(ref err) => {
      error!(
        "Could not import {} (reason {})",
        blk_file_path, err
      );
      // TODO Return error.
    }
  }
}
