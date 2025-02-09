#[derive(Queryable)]
pub struct Output {
    pub id: i64,
    pub output_index: i32,
    pub value: i64,
    pub script: Vec<u8>,
    pub transaction_id: i64,
}
