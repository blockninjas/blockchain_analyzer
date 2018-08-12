use diesel::{self, pg::PgConnection, RunQueryDsl};
use domain::{NewOutput, Output};
use schema::outputs;
use std::result::Result;

pub fn save(
    db_connection: &PgConnection,
    new_output: &NewOutput,
) -> Result<Output, diesel::result::Error> {
    diesel::insert_into(outputs::table)
        .values(new_output)
        .get_result(db_connection)
}
