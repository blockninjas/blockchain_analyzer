extern crate bir;
extern crate bit_vec;
extern crate config;
extern crate db_persistence;
extern crate diesel;
extern crate union_find;
#[macro_use]
extern crate log;

mod cluster_assignment;
mod cluster_unifier;
mod heuristics;

pub use cluster_assignment::ClusterAssignment;
pub use cluster_unifier::ClusterUnifier;
