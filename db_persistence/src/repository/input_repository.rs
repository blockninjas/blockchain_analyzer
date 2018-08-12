use diesel::{self, pg::PgConnection, RunQueryDsl};
use domain::{Input, NewInput};
use schema::inputs;
use std::result::Result;

pub fn save(
    db_connection: &PgConnection,
    new_input: &NewInput,
) -> Result<Input, diesel::result::Error> {
    diesel::insert_into(inputs::table)
        .values(new_input)
        .get_result(db_connection)
}
