#[derive(Queryable)]
pub struct Output {
  pub id: i32,
  pub output_index: i32,
  pub value: i64,
  pub transaction_id: i32,
}
