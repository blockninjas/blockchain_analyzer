use bir::{AddressId, Transaction};
use bit_vec::BitVec;
use std::collections::HashSet;

pub type Cluster = HashSet<AddressId>;

/// A clustering heuristic.
pub trait Heuristic {
    /// Finds address clusters in the given transaction.
    fn cluster_addresses(&self, used_addresses: &BitVec<u32>, transaction: &Transaction)
        -> Cluster;
}
