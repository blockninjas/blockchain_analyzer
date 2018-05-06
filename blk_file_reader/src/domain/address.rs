/// Represents a Bitcoin address.
///
/// For more information, see the according Bitcoin wiki pages on
/// [addresses](https://en.bitcoin.it/wiki/Address) and the
/// [Base58Check encoding](https://en.bitcoin.it/wiki/Base58Check_encoding).
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
  /// The raw 160-bit hash of the Bitcoin address.
  /// TODO Use a wrapper-type to represent address hashes.
  pub hash: [u8; 20],

  /// The Base58Check-encoded `Address::hash`.
  pub base58check: String,
}
