use super::*;
use bincode;
use config::Config;
use db_persistence::repository::BlockRepository;
use diesel::prelude::*;
use std::fs::OpenOptions;
use std::io::BufWriter;
use {Index, Task};

pub struct BirConstructionTask {}

impl BirConstructionTask {
  pub fn new() -> BirConstructionTask {
    BirConstructionTask {}
  }
}

impl Task for BirConstructionTask {
  fn run(&self, config: &Config, db_connection: &PgConnection) {
    info!("Run BirConstructionTask");

    let block_repository = BlockRepository::new(db_connection);

    if let Some(max_block_height) = block_repository.max_height() {
      // TODO Fix possibly truncating cast.
      let max_block_height = max_block_height as u32;

      let bir_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&config.bir_file_path)
        .unwrap();
      let mut bir_file = BufWriter::new(bir_file);

      let mut state =
        state::load_state(&config.bir_construction_state_file_path);

      // TODO Make intent more obvious.
      let number_of_blocks_to_write =
        max_block_height + 1 - state.next_block_height;

      info!(
        "Import {} blocks up to block height {}",
        number_of_blocks_to_write, max_block_height
      );

      construct_bir(&config, &mut state)
        // TODO Fix possibly truncating cast.
        .take(number_of_blocks_to_write as usize)
        .for_each(|block| {
          // TODO Return error instead of panicking.
          bincode::serialize_into(&mut bir_file, &block).unwrap();
        });

      state::save_state(state, &config.bir_construction_state_file_path);
    }

    info!("Finished BirConstructionTask");
  }

  fn get_indexes(&self) -> Vec<Index> {
    vec![]
  }
}
