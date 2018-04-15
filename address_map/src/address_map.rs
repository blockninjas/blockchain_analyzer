pub type AddressHash<'a> = &'a str;
pub type AddressId = u64;

/// Maps address hashes to unique address ids.
pub trait AddressMap {
  /// Get the unique address id that corresponds to the given hash.
  fn get_address_id(&mut self, address_hash: AddressHash) -> AddressId;
}
