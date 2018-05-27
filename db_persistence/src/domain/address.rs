#[derive(Queryable)]
pub struct Address {
  pub id: i64,
  pub base58check: String,
}
