use super::*;
use bincode;
use config::Config;
use db_persistence::repository::BlockRepository;
use diesel::prelude::*;
use failure::Error;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::BufWriter;
use std::path::Path;
use std::result::Result;
use task_manager::{Index, Task};

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
    db_connection: &PgConnection,
  ) -> Result<(), Error> {
    info!("Run BirConstructionTask");

    create_dir_all(&config.unresolved_bir_file_path)?;

    let block_repository = BlockRepository::new(db_connection);

    if let Some(max_block_height) = block_repository.max_height() {
      // TODO Fix possibly truncating cast.
      let max_block_height = max_block_height as u32;

      let mut state =
        state::load_state(&config.bir_construction_state_file_path);

      serialize_bir_into_files(
        &config,
        &mut state,
        db_connection,
        max_block_height,
      );

      state::save_state(state, &config.bir_construction_state_file_path);
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
  max_block_height: u32,
) {
  // TODO Make intent more obvious.
  let number_of_blocks_to_write =
    max_block_height + 1 - state.next_block_height;

  info!(
    "Serialize {} blocks up to block height {}",
    number_of_blocks_to_write, max_block_height
  );

  let mut blocks = construct_bir(config, state, db_connection)
        // TODO Fix possibly truncating cast.
        .take(number_of_blocks_to_write as usize);

  if let Some(block) = blocks.next() {
    let mut bir_file =
      open_bir_file_for_height(&config.unresolved_bir_file_path, block.height);
    for block in blocks {
      if block.height % 10_000 == 0 {
        bir_file = open_bir_file_for_height(
          &config.unresolved_bir_file_path,
          block.height,
        );
      }
      bincode::serialize_into(&mut bir_file, &block).unwrap();
    }
  }
}

fn open_bir_file_for_height<P>(
  bir_file_root_path: P,
  block_height: u32,
) -> BufWriter<File>
where
  P: AsRef<Path>,
{
  let bir_file_name = format!("bir{:05}.dat", block_height / 10_000);
  let bir_file_path = bir_file_root_path.as_ref().join(bir_file_name);
  let bir_file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(bir_file_path)
    .unwrap();
  BufWriter::new(bir_file)
}
