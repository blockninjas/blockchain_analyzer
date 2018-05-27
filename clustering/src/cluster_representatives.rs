use bir::AddressId;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

pub trait ClusterRepresentatives {
  /// Get the cluster-representative of the given address.
  fn get_cluster_representative(&mut self, address: AddressId) -> AddressId;
}

impl ClusterRepresentatives for QuickUnionUf<UnionBySize> {
  fn get_cluster_representative(&mut self, address: AddressId) -> AddressId {
    // TODO Fix possibly truncating cast.
    self.find(address as usize) as u64
  }
}
