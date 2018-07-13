use super::blk_file_importer::*;
use blk_file_reader;
use config::Config;
use db_persistence::repository::*;
use diesel::{self, prelude::*};
use failure::Error;
use rayon::prelude::*;
use std::collections::HashSet;
use std::result::Result;
use task_manager::{Index, Task};

pub struct BlkFileImportTask {}

impl BlkFileImportTask {
  pub fn new() -> BlkFileImportTask {
    BlkFileImportTask {}
  }
}

impl Task for BlkFileImportTask {
  fn run(
    &self,
    config: &Config,
    db_connection: &PgConnection,
  ) -> Result<(), Error> {
    info!("Import blk files");

    let blk_files =
      get_blk_files_to_import(db_connection, &config.blk_file_path);

    // TODO Make number of threads configurable.
    // TODO Handle failing threads.
    blk_files
      .par_iter()
      .for_each(|blk_file| import_blk_file(blk_file, &config.db_url));

    Ok(())
  }

  fn get_indexes(&self) -> Vec<Index> {
    vec![
      Index {
        table: String::from("blocks"),
        column: String::from("hash"),
        unique: false,
      },
      Index {
        table: String::from("blocks"),
        column: String::from("previous_block_hash"),
        unique: false,
      },
      Index {
        table: String::from("transactions"),
        column: String::from("block_id"),
        unique: false,
      },
      Index {
        table: String::from("transactions"),
        column: String::from("hash"),
        unique: false,
      },
      Index {
        table: String::from("inputs"),
        column: String::from("transaction_id"),
        unique: false,
      },
      Index {
        table: String::from("inputs"),
        column: String::from("previous_tx_hash"),
        unique: false,
      },
      Index {
        table: String::from("outputs"),
        column: String::from("transaction_id"),
        unique: false,
      },
      Index {
        table: String::from("output_addresses"),
        column: String::from("base58check"),
        unique: false,
      },
    ]
  }
}

fn get_blk_files_to_import(
  db_connection: &PgConnection,
  blk_file_path: &str,
) -> Vec<String> {
  // Get the blk files that have already been imported by previous runs.
  let blk_file_repository = BlkFileRepository::new(&db_connection);
  let imported_blk_file_names: HashSet<_> =
    blk_file_repository.read_all_names().into_iter().collect();

  // TODO Return error instead of panicking.
  let blk_files = blk_file_reader::read_blk_files(blk_file_path).unwrap();

  // Do not import the latest 2 blk files to be able to ignore blockchain
  // reorganizations.
  // TODO Make this configurable.
  let number_of_files_to_skip_at_end = 2;
  let number_files_to_import = blk_files.len() - number_of_files_to_skip_at_end;

  blk_files
    .into_iter()
    .take(number_files_to_import)
    .filter(|blk_file| {
      !imported_blk_file_names.contains(&get_blk_file_name(blk_file))
    })
    .collect()
}

/// Imports a blk file into the database at `database_url`.
fn import_blk_file(blk_file_path: &str, database_url: &str) {
  info!("Import {}", blk_file_path);

  // TODO Return error instead of panicking.
  let db_connection = PgConnection::establish(database_url).unwrap();
  let transaction_result = db_connection
    .transaction::<(), diesel::result::Error, _>(|| {
      // TODO Return error instead of panicking.
      let blocks = blk_file_reader::read_blocks(blk_file_path).unwrap();
      let blk_file_importer = BlkFileImporter::new(&db_connection);
      blk_file_importer.import(blk_file_path, blocks)
    });

  match transaction_result {
    Ok(_) => {
      info!("Finished import of {}", blk_file_path);
    }
    Err(ref err) => {
      error!("Could not import {} (reason {})", blk_file_path, err);
      // TODO Return error.
    }
  }
}
