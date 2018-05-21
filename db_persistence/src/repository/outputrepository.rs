use super::Repository;
use diesel;
use diesel::RunQueryDsl;
use diesel::pg::PgConnection;
use domain::NewOutput;
use domain::Output;
use schema::outputs;

pub struct OutputRepository<'a> {
  connection: &'a PgConnection,
}

impl<'a> OutputRepository<'a> {
  pub fn new(connection: &'a PgConnection) -> OutputRepository<'a> {
    OutputRepository { connection }
  }
}

impl<'a> Repository for OutputRepository<'a> {
  type NewEntity = NewOutput;
  type Entity = Output;

  fn save(&self, new_output: &NewOutput) -> Output {
    diesel::insert_into(outputs::table)
      .values(new_output)
      .get_result(self.connection)
      .expect("Error saving new output")
  }
}
