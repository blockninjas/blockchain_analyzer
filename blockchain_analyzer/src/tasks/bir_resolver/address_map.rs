use std::collections::HashMap;

pub type AddressId = u64;
pub type Address<'a> = &'a str;

/// Maps base58check addresses to unique address ids.
pub trait AddressMap {
    fn get_id(&self, address: Address) -> AddressId;
    fn get_ids(&self, addresses: &[String]) -> HashMap<String, AddressId>;
}
