use super::Address;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Output {
  pub index: u32,
  pub value: u64,
  pub address: Address,
  pub script: Vec<u8>,
}
