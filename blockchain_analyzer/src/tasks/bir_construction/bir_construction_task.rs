use super::*;
use bincode;
use config::Config;
use db_persistence::repository::BlockRepository;
use diesel::prelude::*;
use failure::Error;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::result::Result;
use task_manager::{Index, Task};

const BLOCK_SIZE: u64 = 1024 * 1024;
const BUFFER_SIZE: u64 = BLOCK_SIZE * 2;
const MAX_BIR_FILE_SIZE: u64 = 1024 * 1024 * 1024;

pub struct BirConstructionTask {}

impl BirConstructionTask {
  pub fn new() -> BirConstructionTask {
    BirConstructionTask {}
  }
}

impl Task for BirConstructionTask {
  fn run(
    &self,
    config: &Config,
    db_connection_pool: &Pool<ConnectionManager<PgConnection>>,
  ) -> Result<(), Error> {
    info!("Run BirConstructionTask");

    create_dir_all(&config.unresolved_bir_file_path)?;

    let db_connection = db_connection_pool.get()?;

    let block_repository = BlockRepository::new(&db_connection);

    if let Some(max_block_height) = block_repository.max_height() {
      // TODO Fix possibly truncating cast.
      let max_block_height = max_block_height as u32;

      let mut state =
        state::load_state(&config.bir_construction_state_file_path);

      // TODO Make intent more obvious.
      let number_of_blocks_to_write =
        max_block_height + 1 - state.next_block_height;

      if number_of_blocks_to_write > 0 {
        info!(
          "Serialize {} blocks up to block height {}",
          number_of_blocks_to_write, max_block_height
        );

        serialize_bir_into_files(
          &config,
          &mut state,
          &db_connection,
          number_of_blocks_to_write,
        )?;

        state::save_state(state, &config.bir_construction_state_file_path);
      }
    }

    info!("Finished BirConstructionTask");

    Ok(())
  }

  fn get_indexes(&self) -> Vec<Index> {
    vec![]
  }
}

fn serialize_bir_into_files(
  config: &Config,
  state: &mut State,
  db_connection: &PgConnection,
  number_of_blocks_to_write: u32,
) -> Result<(), Error> {
  let mut blocks = construct_bir(config, state, db_connection)
        // TODO Fix possibly truncating cast.
        .take(number_of_blocks_to_write as usize);

  if let Some(block) = blocks.next() {
    let unresolved_bir_files =
      bir::read_bir_files(&config.unresolved_bir_file_path)?;

    let mut bir_file_size: u64;
    let mut bir_file_index: usize;

    if let Some(latest_unresolved_bir_file) = unresolved_bir_files.last() {
      bir_file_size = get_bir_file_size(latest_unresolved_bir_file)?;
      bir_file_index = unresolved_bir_files.len() - 1;
    } else {
      bir_file_size = 0;
      bir_file_index = 0;
    };

    let mut bir_file =
      open_bir_file(&config.unresolved_bir_file_path, bir_file_index);

    let mut next_block = Some(block);

    while let Some(block) = next_block {
      let serialized_block = bincode::serialize(&block)?;

      if (bir_file_size + serialized_block.len() as u64) > MAX_BIR_FILE_SIZE {
        bir_file_index += 1;
        bir_file =
          open_bir_file(&config.unresolved_bir_file_path, bir_file_index);
        bir_file_size = 0;
      }

      bir_file.write_all(&serialized_block)?;
      bir_file_size += serialized_block.len() as u64;

      next_block = blocks.next();
    }
  }

  Ok(())
}

fn get_bir_file_size<P>(bir_file_path: P) -> Result<u64, Error>
where
  P: AsRef<Path>,
{
  let bir_file = File::open(bir_file_path)?;
  let metadata = bir_file.metadata()?;
  Ok(metadata.len())
}

fn open_bir_file<P>(bir_file_root_path: P, index: usize) -> BufWriter<File>
where
  P: AsRef<Path>,
{
  let bir_file_name = format!("bir{:05}.dat", index);
  let bir_file_path = bir_file_root_path.as_ref().join(bir_file_name);
  let bir_file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(bir_file_path)
    .unwrap();
  BufWriter::with_capacity(BUFFER_SIZE as usize, bir_file)
}
