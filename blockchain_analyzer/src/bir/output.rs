use super::Address;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Output {
    pub address: Address,
    pub value: u64,
}
