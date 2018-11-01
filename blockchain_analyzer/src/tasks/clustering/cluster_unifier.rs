use super::heuristics::*;
use bir::{self, AddressId, Transaction};
use bit_vec::BitVec;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

/// Finds clusters of addresses.
pub struct ClusterUnifier {
    /// Tracks for each address whether is been used already.
    //TODO BitVec is only implemented for u32 types but addresses are
    // represented using u64.
    used_addresses: BitVec<u32>,

    /// Contains the cluster representative for each address.
    cluster_representatives: QuickUnionUf<UnionBySize>,

    /// The heuristics to drive cluster decisions.
    cluster_heuristics: Vec<Box<Heuristic>>,
}

impl ClusterUnifier {
    pub fn new(max_address_id: AddressId) -> ClusterUnifier {
        // TODO Fix possibly truncating casts.
        ClusterUnifier {
            used_addresses: BitVec::from_elem(max_address_id as usize + 1, false),
            cluster_representatives: QuickUnionUf::<UnionBySize>::new(max_address_id as usize + 1),
            cluster_heuristics: vec![
                Box::new(MultiInputHeuristic {}),
                Box::new(CommonSpendingHeuristic {}),
                Box::new(OneTimeChangeHeuristic {}),
                Box::new(OptimalChangeHeuristic {}),
            ],
        }
    }

    pub fn unify_clusters_in_transactions<T>(&mut self, transactions: T)
    where
        T: Iterator<Item = Transaction>,
    {
        let mut transaction_counter = 0;

        for transaction in transactions {
            let cluster = self.apply_heuristics(&transaction);
            self.unify_with_cluster(&cluster);
            self.mark_addresses_as_used(&transaction);
            transaction_counter += 1;
        }

        info!("Clustered {} transactions", transaction_counter);
    }

    pub fn into_cluster_representatives(mut self) -> Vec<u64> {
        (0..self.cluster_representatives.size())
            .map(|address_id| self.cluster_representatives.find(address_id) as u64)
            .collect()
    }

    fn apply_heuristics(&self, transaction: &Transaction) -> Cluster {
        self.cluster_heuristics
            .iter()
            .map(|heuristic| heuristic.cluster_addresses(&self.used_addresses, transaction))
            .flat_map(|cluster| cluster.into_iter())
            .collect()
    }

    fn unify_with_cluster(&mut self, cluster: &Cluster) {
        let mut addresses: Vec<_> = cluster.into_iter().collect();

        // Sort addresses for consistent behavior over different clustering runs.
        addresses.sort_unstable();

        let mut address_iterator = addresses.into_iter();
        if let Some(&base_address) = address_iterator.next() {
            for &address in address_iterator {
                self.cluster_representatives
                    .union(base_address as usize, address as usize);
            }
        }
    }

    fn mark_addresses_as_used(&mut self, transaction: &Transaction) {
        for input in transaction.inputs.iter() {
            if let bir::Address::Id(address_id) = input.address {
                self.used_addresses.set(address_id as usize, true);
            }
        }

        for output in transaction.outputs.iter() {
            if let bir::Address::Id(address_id) = output.address {
                self.used_addresses.set(address_id as usize, true);
            }
        }
    }
}
