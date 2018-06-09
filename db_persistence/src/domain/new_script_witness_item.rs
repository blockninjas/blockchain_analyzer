use schema::script_witness_items;

#[derive(Insertable)]
#[table_name = "script_witness_items"]
pub struct NewScriptWitnessItem {
  pub input_id: i64,
  pub content: Vec<u8>,
}
