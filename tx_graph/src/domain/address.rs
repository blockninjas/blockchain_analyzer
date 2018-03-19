pub struct Address {
  pub id: u32,

  /// The raw 160-bit hash of the Bitcoin address.
  pub hash: [u8; 20],
}
