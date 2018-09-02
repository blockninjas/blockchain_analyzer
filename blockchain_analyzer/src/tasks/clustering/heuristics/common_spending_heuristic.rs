use super::{Cluster, Heuristic};
use bir::Transaction;
use bit_vec::BitVec;

pub struct CommonSpendingHeuristic {}

impl Heuristic for CommonSpendingHeuristic {
    fn cluster_addresses(
        &self,
        _used_addresses: &BitVec<u32>,
        transaction: &Transaction,
    ) -> Vec<Cluster> {
        let mut clusters = vec![];

        if transaction.outputs.len() == 1 {
            let mut cluster = transaction.get_input_address_ids();
            cluster.extend(transaction.get_output_address_ids().into_iter());
            clusters.push(cluster);
        }

        clusters
    }
}
