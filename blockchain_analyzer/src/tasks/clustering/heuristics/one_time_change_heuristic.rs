use super::{Cluster, Heuristic};
use bir::{AddressId, Transaction};
use bit_vec::BitVec;
use std::collections::HashSet;

pub struct OneTimeChangeHeuristic {}

impl Heuristic for OneTimeChangeHeuristic {
    fn cluster_addresses(
        &self,
        used_addresses: &BitVec<u32>,
        transaction: &Transaction,
    ) -> Cluster {
        let mut cluster = HashSet::new();

        if let Some(change_address) = get_one_time_change_address(transaction, used_addresses) {
            cluster.extend(transaction.get_input_address_ids().into_iter());
            cluster.insert(change_address);
        }

        cluster
    }
}

fn get_one_time_change_address(
    transaction: &Transaction,
    used_addresses: &BitVec<u32>,
) -> Option<AddressId> {
    if contains_self_change_address(transaction) {
        return None;
    }

    let one_time_output_addresses: Vec<u64> = transaction
        .get_output_address_ids()
        .into_iter()
        .filter(|address_id| is_change_address(used_addresses, *address_id))
        .collect();

    if one_time_output_addresses.len() == 1 {
        Some(one_time_output_addresses[0])
    } else {
        None
    }
}

fn contains_self_change_address(transaction: &Transaction) -> bool {
    let input_address_ids: HashSet<u64> = transaction.get_input_address_ids().into_iter().collect();

    transaction
        .get_output_address_ids()
        .iter()
        .any(|address_id| input_address_ids.contains(address_id))
}

/// Returns `true` if the given `address` is a change-address in the given
/// context, `false` otherwise.
fn is_change_address(used_addresses: &BitVec<u32>, address_id: AddressId) -> bool {
    // TODO Fix possibly truncating cast.
    !used_addresses.get(address_id as usize).unwrap()
}
