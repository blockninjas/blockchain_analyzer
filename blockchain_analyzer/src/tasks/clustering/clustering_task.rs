use super::{ClusterAssignment, ClusterUnifier};
use bincode;
use bir;
use config::Config;
use db_persistence::repository::AddressRepository;
use db_persistence::schema::addresses::dsl::*;
use diesel::{self, prelude::*};
use std::fs::File;
use std::io::BufReader;
use task_manager::{Index, Task};

pub struct BirFileIterator {
  pub bir_file: BufReader<File>,
}

impl BirFileIterator {
  pub fn new(bir_file: BufReader<File>) -> BirFileIterator {
    BirFileIterator { bir_file }
  }
}

impl Iterator for BirFileIterator {
  type Item = bir::Block;

  fn next(&mut self) -> Option<bir::Block> {
    if let Ok(block) = bincode::deserialize_from(&mut self.bir_file) {
      Some(block)
    } else {
      None
    }
  }
}

pub struct ClusteringTask {}

impl ClusteringTask {
  pub fn new() -> ClusteringTask {
    ClusteringTask {}
  }
}

impl Task for ClusteringTask {
  fn run(&self, config: &Config, db_connection: &PgConnection) {
    info!("Cluster addresses");

    let bir_file = File::open(&config.bir_file_path).unwrap();
    let bir_file = BufReader::new(bir_file);
    let transactions =
      BirFileIterator::new(bir_file).flat_map(|block| block.transactions);

    // Find clusters and import them into the DB.
    let address_repository = AddressRepository::new(db_connection);
    if let Some(max_address_id) = address_repository.max_id() {
      let max_address_id = max_address_id as u64;
      let cluster_unifier = ClusterUnifier::new(max_address_id);
      let cluster_assignments =
        cluster_unifier.unify_clusters_in_transactions(transactions);
      save_cluster_representatives(db_connection, cluster_assignments);
    };
  }

  fn get_indexes(&self) -> Vec<Index> {
    vec![]
  }
}

fn save_cluster_representatives<C>(
  db_connection: &PgConnection,
  cluster_assignments: C,
) where
  C: IntoIterator<Item = ClusterAssignment>,
{
  info!("Save cluster representatives");

  db_connection.transaction::<(), diesel::result::Error, _>(|| {
    for cluster_assignment in cluster_assignments {
      diesel::update(addresses.filter(id.eq(cluster_assignment.address as i64)))
        .set(cluster_representative.eq(cluster_assignment.cluster_representative as i64))
        .execute(db_connection)
        // TODO Return error instead of panicking.
        .unwrap();
    }
    Ok(())
  })
  // TODO Return error instead of panicking.
  .unwrap();
}
