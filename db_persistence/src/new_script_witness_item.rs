use super::ScriptWitnessItem;
use diesel::{self, pg::PgConnection, RunQueryDsl};
use schema::script_witness_items;

#[derive(Insertable)]
#[table_name = "script_witness_items"]
pub struct NewScriptWitnessItem {
    pub content: Vec<u8>,
    pub input_id: i64,
}

impl NewScriptWitnessItem {
    pub fn save(
        &self,
        db_connection: &PgConnection,
    ) -> Result<ScriptWitnessItem, diesel::result::Error> {
        // TODO Return error instead of panicking.
        diesel::insert_into(script_witness_items::table)
            .values(self)
            .get_result(db_connection)
    }
}
