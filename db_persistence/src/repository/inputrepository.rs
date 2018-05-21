use super::Repository;
use diesel;
use diesel::RunQueryDsl;
use diesel::pg::PgConnection;
use domain::Input;
use domain::NewInput;
use schema::inputs;

pub struct InputRepository<'a> {
  connection: &'a PgConnection,
}

impl<'a> InputRepository<'a> {
  pub fn new(connection: &'a PgConnection) -> InputRepository<'a> {
    InputRepository { connection }
  }
}

impl<'a> Repository for InputRepository<'a> {
  type NewEntity = NewInput;
  type Entity = Input;

  fn save(&self, new_input: &NewInput) -> Input {
    diesel::insert_into(inputs::table)
      .values(new_input)
      .get_result(self.connection)
      .expect("Error saving new input")
  }
}
