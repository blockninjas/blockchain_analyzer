#[derive(Queryable)]
pub struct Transaction {
  pub id: i64,
  pub hash: Vec<u8>,
  pub version: i32,
  pub lock_time: i32,
  pub size_in_bytes: i32,
  pub block_id: i64,
}
