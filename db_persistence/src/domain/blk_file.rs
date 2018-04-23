#[derive(Queryable)]
pub struct BlkFile {
  pub id: i64,
  pub name: String,
  pub number_of_blocks: i32,
}
