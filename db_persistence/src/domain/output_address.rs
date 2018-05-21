#[derive(Queryable)]
pub struct OutputAddress {
  pub output_id: i64,
  pub hash: Vec<u8>,
  pub base58check: String,
}
