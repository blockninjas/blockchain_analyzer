use super::Address;

#[derive(Debug, Clone, PartialEq)]
pub struct Output {
  pub index: u32,
  pub value: u64,
  pub address: Option<Address>,
  pub script: Box<[u8]>,
}
