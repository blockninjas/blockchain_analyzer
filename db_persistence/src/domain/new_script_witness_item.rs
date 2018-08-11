use schema::script_witness_items;

#[derive(Insertable)]
#[table_name = "script_witness_items"]
pub struct NewScriptWitnessItem {
    pub content: Vec<u8>,
    pub input_id: i64,
}
