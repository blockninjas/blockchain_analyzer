use super::ClusterUnifier;
use bir;
use config::Config;
use db_persistence::repository::AddressRepository;
use db_persistence::schema;
use diesel::{self, prelude::*};
use failure::Error;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::result::Result;
use std::sync::Mutex;
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
    db_connection_pool: &Pool<ConnectionManager<PgConnection>>,
  ) -> Result<(), Error> {
    info!("Run ClusteringTask");

    let transactions = bir::read_bir_files(&config.resolved_bir_file_path)?
      .into_iter()
      .map(|path| File::open(path).unwrap()) // TODO Return error instead of panicking.
      .map(|bir_file| BufReader::new(bir_file))
      .flat_map(|bir_file| bir::BirFileIterator::new(bir_file))
      .flat_map(|block| block.transactions);

    // Find clusters and import them into the DB.
    let max_address_id = {
      let db_connection = db_connection_pool.get()?;
      let address_repository = AddressRepository::new(&db_connection);
      address_repository.max_id()?
    };

    if let Some(max_address_id) = max_address_id {
      let max_address_id = max_address_id as u64;
      let mut cluster_unifier = ClusterUnifier::new(max_address_id);
      cluster_unifier.unify_clusters_in_transactions(transactions);
      let cluster_representatives =
        cluster_unifier.into_cluster_representatives();
      save_cluster_representatives(
        db_connection_pool,
        &cluster_representatives,
      );
    };

    info!("Finished ClusteringTask");

    Ok(())
  }

  fn get_indexes(&self) -> Vec<Index> {
    vec![]
  }
}

fn save_cluster_representatives(
  db_connection_pool: &Pool<ConnectionManager<PgConnection>>,
  cluster_representatives: &[usize],
) {
  info!("Save cluster representatives");

  let update_counter = Mutex::new(0);

  // TODO Handle inconsistency during updates in parallel transactions.
  cluster_representatives
    .par_iter()
    .enumerate()
    .chunks(1_000_000)
    .for_each(|cluster_assignments| {
      let mut number_of_assignments = 0;

      // TODO Return error instead of panicking.
      let db_connection = db_connection_pool.get().unwrap();

      db_connection
        .transaction::<(), diesel::result::Error, _>(|| {
          for (address_id, cluster_representative) in cluster_assignments {
            let cluster_representative = *cluster_representative as i64;

            // TODO Can `UnionJoin::find()` return `0`?
            let cluster_representative = if cluster_representative > 0 {
              Some(cluster_representative)
            } else {
              None
            };

            number_of_assignments += update_cluster_representative(
              &db_connection,
              address_id as i64,
              cluster_representative,
            ).unwrap();
          }
          Ok(())
        })
        .unwrap();

      let mut update_counter = update_counter.lock().unwrap();
      *update_counter += number_of_assignments;
      info!("Saved {} cluster representatives", update_counter);
    });
}

fn update_cluster_representative(
  db_connection: &PgConnection,
  address_id: i64,
  cluster_representative: Option<i64>,
) -> Result<usize, diesel::result::Error> {
  diesel::update(
    schema::addresses::dsl::addresses
      .filter(schema::addresses::dsl::id.eq(address_id)),
  ).set(
    schema::addresses::dsl::cluster_representative.eq(cluster_representative),
  )
    .execute(db_connection)
}
