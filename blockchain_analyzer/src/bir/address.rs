use super::AddressId;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Address {
    UnresolvedAddress,
    Id(AddressId),
    Base58Check(String),
}

impl Address {
    pub fn to_address_id(&self) -> Option<AddressId> {
        if let Address::Id(address_id) = *self {
            Some(address_id)
        } else {
            None
        }
    }
}
