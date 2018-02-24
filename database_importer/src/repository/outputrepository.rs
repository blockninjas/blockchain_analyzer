use super::Repository;
use ::domain::Output;
use ::domain::NewOutput;
use ::schema::outputs;
use ::schema::outputs::dsl::*;
use diesel;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;

pub struct OutputRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> OutputRepository<'a> {
    fn new(connection: &'a PgConnection) -> OutputRepository<'a> {
        OutputRepository {
            connection
        }
    }
}

impl<'a> Repository for OutputRepository<'a> {
    type NewEntity = NewOutput;
    type Entity = Output;

    fn save(&self, new_block: &NewOutput) -> Output {
        diesel::insert_into(outputs::table)
            .values(new_block)
            .get_result(self.connection)
            .expect("Error saving new output")
    }
}