use bir::{AddressId, Transaction};
use bit_vec::BitVec;

/// A cluster of addresses.
pub type Cluster = Vec<AddressId>;

/// A clustering heuristic.
pub trait Heuristic {
    /// Finds address clusters in the given transaction.
    fn cluster_addresses(
        &self,
        used_addresses: &BitVec<u32>,
        transaction: &Transaction,
    ) -> Vec<Cluster>;
}
