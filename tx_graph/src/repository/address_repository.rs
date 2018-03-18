use std::io;
use domain::Address;

pub trait AddressRepository {
  fn save(address: &Address) -> io::Result<()>;
}
