extern crate bir;
extern crate bir_construction;
extern crate bit_vec;
extern crate config;
extern crate db_persistence;
extern crate diesel;
extern crate union_find;

mod cluster_representatives;
mod cluster_unifier;

use cluster_representatives::ClusterRepresentatives;
use cluster_unifier::ClusterUnifier;
use config::Config;
use db_persistence::repository::AddressRepository;
use db_persistence::schema::cluster_representatives::dsl::*;
use diesel::{Connection, ExpressionMethods, PgConnection, RunQueryDsl};

/// Computes clusters of addresses based on the given `Config`.
pub fn compute_clusters(config: &Config) {
  let db_connection = PgConnection::establish(&config.db_url).unwrap();

  // Normalize blk files into BIR.
  let bir = bir_construction::construct_bir(&config, &db_connection);

  // Find clusters and import them into the DB.
  let address_repository = AddressRepository::new(&db_connection);
  if let Some(max_address_id) = address_repository.max_id() {
    let max_address_id = max_address_id as u64;
    let cluster_unifier = ClusterUnifier::new(max_address_id);
    let clusters = cluster_unifier.unify_clusters_in_blocks(bir);
    save_cluster_representatives(&db_connection, clusters, max_address_id);
  }
}

fn save_cluster_representatives<C>(
  db_connection: &PgConnection,
  mut clusters: C,
  max_address_id: u64,
) where
  C: ClusterRepresentatives,
{
  db_connection
    .transaction::<(), diesel::result::Error, _>(|| {

  // TODO Do not assume that address ids are consecutive.
  for address_id in 1..max_address_id + 1 {
    let cluster_representative =
      clusters.get_cluster_representative(address_id);

    // TODO Extract into `sql_function`.
    diesel::insert_into(db_persistence::schema::cluster_representatives::table)
      .values((
        address.eq(address_id as i64),
        representative.eq(cluster_representative as i64),
      ))
      .execute(db_connection)
      .unwrap();
  }
      Ok(())
    })
    // TODO Return error instead of panicking.
    .unwrap();
}
