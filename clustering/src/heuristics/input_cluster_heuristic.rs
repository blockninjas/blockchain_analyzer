use super::{Cluster, Heuristic};
use bir::{ResolvedAddress, Transaction};
use bit_vec::BitVec;

pub struct InputClusterHeuristic {}

impl InputClusterHeuristic {
  pub fn new() -> InputClusterHeuristic {
    InputClusterHeuristic {}
  }
}

impl Heuristic for InputClusterHeuristic {
  fn cluster_addresses(
    &self,
    _used_addresses: &BitVec<u32>,
    transaction: &Transaction,
  ) -> Vec<Cluster> {
    let input_cluster: Cluster = transaction
      .inputs
      .iter()
      .filter_map(|input| {
        if let ResolvedAddress { address_id } = input.address {
          Some(address_id)
        } else {
          None
        }
      })
      .collect();
    vec![input_cluster]
  }
}
