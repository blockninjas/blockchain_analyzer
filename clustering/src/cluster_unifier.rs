use super::ClusterRepresentatives;
use bir::{AddressId, Block, Transaction};
use bit_vec::BitVec;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

/// Finds clusters of addresses.
pub struct ClusterUnifier {
  /// Tracks for each address whether is been used already.
  //TODO BitVec is only implemented for u32 types but addresses are
  // represented using u64.
  pub used_addresses: BitVec<u32>,

  /// Contains the cluster representative for each address.
  pub cluster_representatives: QuickUnionUf<UnionBySize>,
}

/// A one-time change transaction.
struct OtcTransaction {
  pub input_addresses: Vec<AddressId>,
  pub receiver_address: AddressId,
  pub change_address: AddressId,
}

type Cluster = Vec<AddressId>;

impl ClusterUnifier {
  /// Creates a new `ClusterUnifier`.
  pub fn new(max_address_id: AddressId) -> ClusterUnifier {
    // TODO Fix possibly truncating casts.
    ClusterUnifier {
      used_addresses: BitVec::from_elem(max_address_id as usize + 1, false),
      cluster_representatives: QuickUnionUf::<UnionBySize>::new(
        max_address_id as usize + 1,
      ),
    }
  }

  /// Unifies clusters of addresses in the given `blocks`.
  ///
  /// Returns the resulting cluster representatives of the .
  pub fn unify_clusters_in_blocks<B>(
    mut self,
    blocks: B,
  ) -> impl ClusterRepresentatives
  where
    B: Iterator<Item = Block>,
  {
    blocks
      .flat_map(|block| block.transactions)
      .for_each(|transaction| {
        self.unify_clusters_in_transaction(&transaction);
      });
    self.cluster_representatives
  }

  /// Unifies clusters of addresses in the given `transaction`.
  fn unify_clusters_in_transaction(&mut self, transaction: &Transaction) {
    let clusters = self.find_clusters_in_transaction(transaction);

    self.record_cluster_representatives(&clusters);

    for input in transaction.inputs.iter() {
      self
        .used_addresses
        .set(input.address_id as usize, true);
    }

    for output in transaction.outputs.iter() {
      self
        .used_addresses
        .set(output.address_id as usize, true);
    }
  }

  /// Finds clusters in the given `transaction`.
  fn find_clusters_in_transaction(
    &self,
    transaction: &Transaction,
  ) -> Vec<Cluster> {
    if let Some(otc_transaction) = self.to_otc_transaction(transaction) {
      // "One-time change" heuristic.
      let mut sender_cluster = otc_transaction.input_addresses;
      sender_cluster.push(otc_transaction.change_address);
      let receiver_cluster = vec![otc_transaction.receiver_address];
      vec![sender_cluster, receiver_cluster]
    } else if transaction.outputs.len() == 1 && transaction.inputs.len() > 1 {
      // "Common-spending" heuristic.
      let mut cluster: Cluster = transaction
        .inputs
        .iter()
        .map(|input| input.address_id)
        .collect();
      cluster.push(transaction.outputs[0].address_id);
      vec![cluster]
    } else {
      // If no other heuristic applies, assume that all input-addresses form a
      // cluster.
      let mut input_cluster: Cluster = transaction
        .inputs
        .iter()
        .map(|input| input.address_id)
        .collect();
      vec![input_cluster]
    }
  }

  /// Aligns the current cluster representatives with the given clusters.
  fn record_cluster_representatives(&mut self, clusters: &[Cluster]) {
    for cluster in clusters {
      if cluster.len() > 1 {
        let base_address = cluster[0];
        for &address in cluster.iter().skip(1) {
          self
            .cluster_representatives
            .union(base_address as usize, address as usize);
        }
      }
    }
  }

  /// Transforms the given transaction to an OtcTransaction.
  ///
  /// Returns `Some` if the given `transaction` is an `OtcTransaction` in the
  /// current context, `None` otherwise.
  fn to_otc_transaction(
    &self,
    transaction: &Transaction,
  ) -> Option<OtcTransaction> {
    if transaction.inputs.len() != 2 {
      None
    } else {
      let address0 = transaction.inputs[0].address_id;
      let address1 = transaction.inputs[1].address_id;

      let (change_address, receiver_address): (AddressId, AddressId) = if self
        .is_change_address(address0)
        && !self.is_change_address(address1)
      {
        (address0, address1)
      } else if !self.is_change_address(address0)
        && self.is_change_address(address1)
      {
        (address1, address0)
      } else {
        return None;
      };

      let otc_transaction = OtcTransaction {
        input_addresses: transaction
          .inputs
          .iter()
          .map(|input| input.address_id)
          .collect(),
        receiver_address,
        change_address,
      };

      Some(otc_transaction)
    }
  }

  /// Returns `true` if the given `address` is a change-address in the given
  /// context, `false` otherwise.
  fn is_change_address(&self, address: AddressId) -> bool {
    // TODO Fix possibly truncating cast.
    self
      .used_addresses
      .get(address as usize)
      .unwrap()
  }
}
