extern crate bir;
extern crate bir_construction;
extern crate bit_vec;
extern crate blk_file_reader;
extern crate config;
extern crate db_persistence;
extern crate diesel;
extern crate union_find;

mod cluster_assignment;
mod cluster_unifier;

use cluster_assignment::ClusterAssignment;
use cluster_unifier::ClusterUnifier;
use config::Config;
use db_persistence::repository::AddressRepository;
use db_persistence::schema::cluster_representatives::dsl::*;
use diesel::{Connection, ExpressionMethods, PgConnection, RunQueryDsl};

/// Computes clusters of addresses based on the given `Config`.
pub fn compute_clusters<B>(config: &Config, blocks: B)
where
  B: IntoIterator<Item = blk_file_reader::Block>,
{
  let db_connection = PgConnection::establish(&config.db_url).unwrap();

  // Normalize blk files into BIR.
  let bir = bir_construction::construct_bir(&config, &db_connection, blocks);

  // Find clusters and import them into the DB.
  let address_repository = AddressRepository::new(&db_connection);
  if let Some(max_address_id) = address_repository.max_id() {
    let max_address_id = max_address_id as u64;
    let cluster_unifier = ClusterUnifier::new(max_address_id);
    let cluster_assignments = cluster_unifier.unify_clusters_in_blocks(bir);
    save_cluster_representatives(&db_connection, cluster_assignments);
  }
}

fn save_cluster_representatives<C>(
  db_connection: &PgConnection,
  cluster_assignments: C,
) where
  C: IntoIterator<Item = ClusterAssignment>,
{
  db_connection.transaction::<(), diesel::result::Error, _>(|| {
    // TODO Return error instead of panicking.
    diesel::delete(db_persistence::schema::cluster_representatives::table)
      .execute(db_connection)
      .unwrap();

    for cluster_assignment in cluster_assignments {
      diesel::insert_into(db_persistence::schema::cluster_representatives::table)
        .values((
            address.eq(cluster_assignment.address as i64),
            representative.eq(cluster_assignment.cluster_representative as i64),
         ))
         .execute(db_connection)
         // TODO Return error instead of panicking.
         .unwrap();
    }

    Ok(())
  })
  // TODO Return error instead of panicking.
  .unwrap();
}
