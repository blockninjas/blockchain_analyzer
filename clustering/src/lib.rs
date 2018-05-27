extern crate bir;
extern crate bir_construction;
extern crate bit_vec;
extern crate config;
extern crate union_find;

mod cluster_representatives;
mod cluster_unifier;

use cluster_representatives::ClusterRepresentatives;
use cluster_unifier::ClusterUnifier;
use config::Config;

/// Computes clusters of addresses based on the given `Config`.
pub fn compute_clusters(config: &Config) {
  let bir = bir_construction::construct_bir(&config);
  let number_of_addresses = 100_000; // TODO Extract this from database.
  let cluster_unifier = ClusterUnifier::new(number_of_addresses);
  let _cluster_representatives = cluster_unifier.unify_clusters_in_blocks(bir);
  // TODO Import cluster representatives into database.
}
