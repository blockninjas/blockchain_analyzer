use super::AddressId;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Address {
  UnresolvedAddress,
  ResolvedAddress { address_id: AddressId },
}
