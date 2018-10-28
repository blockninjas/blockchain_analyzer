use super::{Cluster, Heuristic};
use bir::Transaction;
use bit_vec::BitVec;
use std::collections::HashSet;

pub struct CommonSpendingHeuristic {}

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
