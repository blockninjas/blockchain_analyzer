use diesel::{self, pg::PgConnection, RunQueryDsl};
use domain::NewScriptWitnessItem;
use schema::script_witness_items;

pub struct ScriptWitnessItemRepository<'a> {
  connection: &'a PgConnection,
}

impl<'a> ScriptWitnessItemRepository<'a> {
  pub fn new(connection: &'a PgConnection) -> ScriptWitnessItemRepository<'a> {
    ScriptWitnessItemRepository { connection }
  }

  pub fn save(&self, new_script_witness_item: &NewScriptWitnessItem) {
    // TODO Return error instead of panicking.
    diesel::insert_into(script_witness_items::table)
      .values(new_script_witness_item)
      .execute(self.connection)
      .expect("Error saving new script witness item");
  }
}
