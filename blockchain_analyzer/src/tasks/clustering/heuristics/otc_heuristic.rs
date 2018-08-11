use super::{Cluster, Heuristic};
use bir::{Address, AddressId, Transaction};
use bit_vec::BitVec;

#[allow(dead_code)]
pub struct OtcHeuristic {}

impl OtcHeuristic {
    #[allow(dead_code)]
    pub fn new() -> OtcHeuristic {
        OtcHeuristic {}
    }
}

impl Heuristic for OtcHeuristic {
    fn cluster_addresses(
        &self,
        used_addresses: &BitVec<u32>,
        transaction: &Transaction,
    ) -> Vec<Cluster> {
        let mut clusters = vec![];
        if let Some(otc_transaction) = to_otc_transaction(used_addresses, transaction) {
            // "One-time change" heuristic.
            let mut sender_cluster = otc_transaction.input_addresses;
            sender_cluster.push(otc_transaction.change_address);
            let receiver_cluster = vec![otc_transaction.receiver_address];
            clusters.push(sender_cluster);
            clusters.push(receiver_cluster);
        }
        clusters
    }
}

/// A one-time change transaction.
struct OtcTransaction {
    pub input_addresses: Vec<AddressId>,
    pub receiver_address: AddressId,
    pub change_address: AddressId,
}

/// Transforms the given transaction to an OtcTransaction.
///
/// Returns `Some` if the given `transaction` is an `OtcTransaction` in the
/// current context, `None` otherwise.
fn to_otc_transaction(
    used_addresses: &BitVec<u32>,
    transaction: &Transaction,
) -> Option<OtcTransaction> {
    if transaction.outputs.len() != 2 {
        None
    } else {
        let address0 = match transaction.outputs[0].address {
            Address::Id(address_id) => address_id,
            _ => return None,
        };

        let address1 = match transaction.outputs[1].address {
            Address::Id(address_id) => address_id,
            _ => return None,
        };

        let (change_address, receiver_address): (AddressId, AddressId) =
            if is_change_address(used_addresses, address0)
                && !is_change_address(used_addresses, address1)
            {
                (address0, address1)
            } else if !is_change_address(used_addresses, address0)
                && is_change_address(used_addresses, address1)
            {
                (address1, address0)
            } else {
                return None;
            };

        let otc_transaction = OtcTransaction {
            input_addresses: transaction
                .inputs
                .iter()
                .filter_map(|input| {
                    if let Address::Id(address_id) = input.address {
                        Some(address_id)
                    } else {
                        None
                    }
                })
                .collect(),
            receiver_address,
            change_address,
        };

        Some(otc_transaction)
    }
}

/// Returns `true` if the given `address` is a change-address in the given
/// context, `false` otherwise.
fn is_change_address(used_addresses: &BitVec<u32>, address_id: AddressId) -> bool {
    // TODO Fix possibly truncating cast.
    !used_addresses.get(address_id as usize).unwrap()
}
