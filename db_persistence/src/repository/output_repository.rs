use diesel::{self, pg::PgConnection, RunQueryDsl};
use domain::{NewOutput, Output};
use schema::outputs;
use std::result::Result;

pub struct OutputRepository<'a> {
  connection: &'a PgConnection,
}

impl<'a> OutputRepository<'a> {
  pub fn new(connection: &'a PgConnection) -> OutputRepository<'a> {
    OutputRepository { connection }
  }

  pub fn save(
    &self,
    new_output: &NewOutput,
  ) -> Result<Output, diesel::result::Error> {
    diesel::insert_into(outputs::table)
      .values(new_output)
      .get_result(self.connection)
  }
}
