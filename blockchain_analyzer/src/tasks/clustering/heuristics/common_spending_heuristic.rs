use super::{Cluster, Heuristic};
use bir::{Address, Transaction};
use bit_vec::BitVec;

pub struct CommonSpendingHeuristic {}

impl CommonSpendingHeuristic {
  pub fn new() -> CommonSpendingHeuristic {
    CommonSpendingHeuristic {}
  }
}

impl Heuristic for CommonSpendingHeuristic {
  fn cluster_addresses(
    &self,
    _used_addresses: &BitVec<u32>,
    transaction: &Transaction,
  ) -> Vec<Cluster> {
    let mut cluster: Cluster = transaction
      .inputs
      .iter()
      .filter_map(|input| {
        if let Address::Id(address_id) = input.address {
          Some(address_id)
        } else {
          None
        }
      })
      .collect();

    if let Address::Id(address_id) = transaction.outputs[0].address {
      cluster.push(address_id);
    }
    vec![cluster]
  }
}
