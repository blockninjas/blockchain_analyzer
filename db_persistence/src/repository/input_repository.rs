use diesel::{self, pg::PgConnection, RunQueryDsl};
use domain::{Input, NewInput};
use schema::inputs;
use std::result::Result;

pub struct InputRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> InputRepository<'a> {
    pub fn new(connection: &'a PgConnection) -> InputRepository<'a> {
        InputRepository { connection }
    }

    pub fn save(&self, new_input: &NewInput) -> Result<Input, diesel::result::Error> {
        diesel::insert_into(inputs::table)
            .values(new_input)
            .get_result(self.connection)
    }
}
