use data_encoding::HEXLOWER;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Hash(pub [u8; 32]);

impl fmt::Debug for Hash {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", HEXLOWER.encode(&self.0))
  }
}
