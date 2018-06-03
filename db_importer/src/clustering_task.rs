use super::{Index, Task};
use blk_file_reader;
use clustering;
use config::Config;
use diesel::prelude::*;

pub struct ClusteringTask {}

impl ClusteringTask {
  pub fn new() -> ClusteringTask {
    ClusteringTask {}
  }
}

impl Task for ClusteringTask {
  fn run(&self, config: &Config, _db_connection: &PgConnection) {
    info!("Cluster addresses");

    // TODO Only cluster blk files that have been imported successfully.
    // TODO Only cluster up to maximum persisted block height.
    let blk_files =
      blk_file_reader::read_blk_files(&config.blk_file_path).unwrap();
    let number_of_blk_files_to_cluster = blk_files.len() - 2;
    let blk_files_to_cluster = blk_files
      .into_iter()
      .take(number_of_blk_files_to_cluster);

    let blocks_to_cluster = blk_files_to_cluster
      .flat_map(|blk_file_path| {
        blk_file_reader::read_blocks(&blk_file_path).unwrap()
      })
      .map(|block| block.unwrap());

    clustering::compute_clusters(&config, blocks_to_cluster);
  }

  fn get_indexes(&self) -> Vec<Index> {
    vec![]
  }
}
