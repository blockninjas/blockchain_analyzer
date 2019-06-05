use super::{Cluster, Heuristic};
use bir::Transaction;
use bit_vec::BitVec;
use std::collections::HashSet;

pub struct CommonSpendingHeuristic {}

/// Common-Spending Heuristic
///
/// If a transaction has exactly one output, all addressess that are part of this transaction are
/// considered to be controlled by the same person.
impl Heuristic for CommonSpendingHeuristic {
    fn cluster_addresses(
        &self,
        _used_addresses: &BitVec<u32>,
        transaction: &Transaction,
    ) -> Cluster {
        let mut cluster = HashSet::new();

        if transaction.outputs.len() == 1 {
            cluster.extend(transaction.get_input_address_ids().into_iter());
            cluster.extend(transaction.get_output_address_ids().into_iter());
        }

        cluster
    }
}
