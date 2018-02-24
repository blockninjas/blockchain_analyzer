#[derive(Queryable)]
pub struct Address {
    pub id: i32,
    pub hash: Vec<u8>,
    pub base58_string: String,
}
