use std::fmt;

pub struct Hash(pub [u8; 32]);

impl fmt::Debug for Hash {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_hex_string())
  }
}

impl Hash {
  pub fn to_hex_string(&self) -> String {
    to_hex_string(&self.0)
  }
}

fn to_hex_string(bytes: &[u8]) -> String {
  bytes.iter().map(|b| format!("{:02X}", b)).collect()
}

#[cfg(test)]
mod to_hex_string_tests {
  use super::to_hex_string;

  #[test]
  fn returns_big_endian_hex() {
    // given
    let bytes = [0x89, 0xAB, 0xCD, 0xEF];

    // when
    let actual_hex = to_hex_string(&bytes);

    // then
    let expected_hex = "89ABCDEF";
    assert_eq!(expected_hex, actual_hex);
  }

  #[test]
  fn does_not_truncate_leading_zeros() {
    // given
    let bytes = [0x00, 0x01];

    // when
    let actual_hex = to_hex_string(&bytes);

    // then
    let expected_hex = "0001";
    assert_eq!(expected_hex, actual_hex);
  }
}
