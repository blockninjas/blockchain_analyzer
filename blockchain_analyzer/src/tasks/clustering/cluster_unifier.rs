use super::{heuristics::*, ClusterAssignment};
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
    /// Creates a new `ClusterUnifier`.
    pub fn new(max_address_id: AddressId) -> ClusterUnifier {
        // TODO Fix possibly truncating casts.
        ClusterUnifier {
            used_addresses: BitVec::from_elem(max_address_id as usize + 1, false),
            cluster_representatives: QuickUnionUf::<UnionBySize>::new(max_address_id as usize + 1),
            cluster_heuristics: vec![
                // TODO Fix classification of change-addresses in OtcHeuristic.
                // Box::new(OtcHeuristic::new()),
                // Box::new(CommonSpendingHeuristic::new()),
                Box::new(InputClusterHeuristic::new()),
            ],
        }
    }

    /// Unifies clusters of addresses in the given `blocks`.
    ///
    /// Returns the resulting cluster representatives of the .
    pub fn unify_clusters_in_transactions<T>(&mut self, transactions: T)
    where
        T: Iterator<Item = Transaction>,
    {
        let mut transaction_counter = 0;

        for transaction in transactions {
            self.unify_clusters_in_transaction(&transaction);
            transaction_counter += 1;
        }

        info!("Clustered {} transactions", transaction_counter);
    }

    pub fn into_cluster_representatives(mut self) -> Vec<usize> {
        (0..self.cluster_representatives.size())
            .map(|address_id| self.cluster_representatives.find(address_id))
            .collect()
    }

    /// Unifies clusters of addresses in the given `transaction`.
    fn unify_clusters_in_transaction(&mut self, transaction: &Transaction) {
        let clusters = self.find_clusters_in_transaction(transaction);

        self.record_cluster_representatives(&clusters);

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

    /// Finds clusters in the given `transaction`.
    fn find_clusters_in_transaction(&self, transaction: &Transaction) -> Vec<Cluster> {
        let mut clusters = vec![];
        for heuristic in self.cluster_heuristics.iter() {
            let mut heuristic_clusters =
                heuristic.cluster_addresses(&self.used_addresses, transaction);
            clusters.append(&mut heuristic_clusters);
        }
        clusters
    }

    /// Aligns the current cluster representatives with the given clusters.
    fn record_cluster_representatives(&mut self, clusters: &[Cluster]) {
        for cluster in clusters {
            if cluster.len() > 1 {
                let base_address = cluster[0];
                for &address in cluster.iter().skip(1) {
                    self.cluster_representatives
                        .union(base_address as usize, address as usize);
                }
            }
        }
    }
}
