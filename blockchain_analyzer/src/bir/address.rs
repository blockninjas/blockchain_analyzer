use super::AddressId;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Address {
  UnresolvedAddress,
  Id(AddressId),
  Base58Check(String),
}
