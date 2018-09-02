use super::{Cluster, Heuristic};
use bir::{Address, Transaction};
use bit_vec::BitVec;

pub struct OptimalChangeHeuristic {}

impl Heuristic for OptimalChangeHeuristic {
    fn cluster_addresses(
        &self,
        _used_addresses: &BitVec<u32>,
        transaction: &Transaction,
    ) -> Vec<Cluster> {
        if transaction.inputs.len() == 0 {
            return vec![];
        }

        let smallest_input_value = transaction
            .inputs
            .iter()
            .map(|input| input.value)
            .min()
            .unwrap();

        let change_address_candidates: Vec<Address> = transaction
            .outputs
            .iter()
            .filter(|output| output.value < smallest_input_value)
            .map(|output| output.address.clone())
            .collect();

        let mut clusters = vec![];
        if change_address_candidates.len() == 1 {
            if let Address::Id(address_id) = change_address_candidates[0] {
                clusters.push(vec![address_id]);
            }
        }
        clusters
    }
}
