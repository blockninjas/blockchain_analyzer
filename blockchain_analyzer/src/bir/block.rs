use super::Transaction;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Block {
    pub height: u32,
    pub transactions: Vec<Transaction>,
}
