use std::io;

pub trait AddressRepository {
  /// Saves the address with the given hash if it does not already exist.
  ///
  /// In either case returns the id of the newly saved or existing address.
  fn save_if_not_exists(hash: [u8; 20]) -> io::Result<u32>;
}
