use diesel::{self, RunQueryDsl, pg::PgConnection};
use domain::{NewOutput, Output};
use schema::outputs;

pub struct OutputRepository<'a> {
  connection: &'a PgConnection,
}

impl<'a> OutputRepository<'a> {
  pub fn new(connection: &'a PgConnection) -> OutputRepository<'a> {
    OutputRepository { connection }
  }

  pub fn save(&self, new_output: &NewOutput) -> Output {
    diesel::insert_into(outputs::table)
      .values(new_output)
      .get_result(self.connection)
      .expect("Error saving new output")
  }
}
