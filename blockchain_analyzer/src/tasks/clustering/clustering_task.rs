use super::{ClusterAssignment, ClusterUnifier};
use bir;
use config::Config;
use db_persistence::repository::AddressRepository;
use db_persistence::schema::addresses::dsl::*;
use diesel::{self, prelude::*};
use failure::Error;
use std::fs::File;
use std::io::BufReader;
use std::result::Result;
use task_manager::{Index, Task};

pub struct ClusteringTask {}

impl ClusteringTask {
  pub fn new() -> ClusteringTask {
    ClusteringTask {}
  }
}

impl Task for ClusteringTask {
  fn run(
    &self,
    config: &Config,
    db_connection: &PgConnection,
  ) -> Result<(), Error> {
    info!("Run ClusteringTask");

    let transactions = bir::read_bir_files(&config.resolved_bir_file_path)?
      .into_iter()
      .map(|path| File::open(path).unwrap()) // TODO Return error instead of panicking.
      .map(|bir_file| BufReader::new(bir_file))
      .flat_map(|bir_file| bir::BirFileIterator::new(bir_file))
      .flat_map(|block| block.transactions);

    // Find clusters and import them into the DB.
    let address_repository = AddressRepository::new(db_connection);
    if let Some(max_address_id) = address_repository.max_id() {
      let max_address_id = max_address_id as u64;
      let cluster_unifier = ClusterUnifier::new(max_address_id);
      let cluster_assignments =
        cluster_unifier.unify_clusters_in_transactions(transactions);
      save_cluster_representatives(db_connection, cluster_assignments);
    };

    info!("Finished ClusteringTask");

    Ok(())
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
