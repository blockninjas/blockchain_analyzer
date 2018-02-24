#[derive(Queryable)]
pub struct Address {
    pub id: i32,
    pub value: Vec<u8>,
}
