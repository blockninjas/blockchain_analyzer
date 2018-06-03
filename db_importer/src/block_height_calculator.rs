use super::{Index, Task};
use config::Config;
use db_persistence::repository::*;
use diesel::prelude::*;

pub struct BlockHeightCalculator {}

impl BlockHeightCalculator {
  pub fn new() -> BlockHeightCalculator {
    BlockHeightCalculator {}
  }
}

impl Task for BlockHeightCalculator {
  fn run(&self, _config: &Config, db_connection: &PgConnection) {
    info!("Calculate block height");
    let block_repository = BlockRepository::new(db_connection);
    let _ = block_repository.calculate_block_height();
  }

  fn get_indexes(&self) -> Vec<Index> {
    vec![Index {
      table: String::from("blocks"),
      column: String::from("height"),
      unique: false,
    }]
  }
}
