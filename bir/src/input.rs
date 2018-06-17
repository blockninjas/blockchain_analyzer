use super::Address;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Input {
  pub address: Address,
  pub value: u64,
}
