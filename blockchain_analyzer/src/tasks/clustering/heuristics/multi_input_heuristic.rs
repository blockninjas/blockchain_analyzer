use super::{Cluster, Heuristic};
use bir::Transaction;
use bit_vec::BitVec;

pub struct MultiInputHeuristic {}

impl Heuristic for MultiInputHeuristic {
    fn cluster_addresses(
        &self,
        _used_addresses: &BitVec<u32>,
        transaction: &Transaction,
    ) -> Cluster {
        transaction.get_input_address_ids().into_iter().collect()
    }
}
