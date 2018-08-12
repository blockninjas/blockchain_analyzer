use diesel::{self, pg::PgConnection, RunQueryDsl};
use domain::{NewScriptWitnessItem, ScriptWitnessItem};
use schema::script_witness_items;

pub fn save(
    db_connection: &PgConnection,
    new_script_witness_item: &NewScriptWitnessItem,
) -> Result<ScriptWitnessItem, diesel::result::Error> {
    // TODO Return error instead of panicking.
    diesel::insert_into(script_witness_items::table)
        .values(new_script_witness_item)
        .get_result(db_connection)
}
