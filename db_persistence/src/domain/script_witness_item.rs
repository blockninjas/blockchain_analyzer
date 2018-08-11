#[derive(Queryable)]
pub struct ScriptWitnessItem {
  pub id: i64,
  pub content: Vec<u8>,
  pub input_id: i64,
}
