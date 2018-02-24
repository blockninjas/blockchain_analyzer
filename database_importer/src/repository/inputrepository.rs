use super::Repository;
use ::domain::Input;
use ::domain::NewInput;
use ::schema::inputs;
use diesel;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;

pub struct InputRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> InputRepository<'a> {
    pub fn new(connection: &'a PgConnection) -> InputRepository<'a> {
        InputRepository {
            connection
        }
    }
}

impl<'a> Repository for InputRepository<'a> {
    type NewEntity = NewInput;
    type Entity = Input;

    fn save(&self, new_block: &NewInput) -> Input {
        diesel::insert_into(inputs::table)
            .values(new_block)
            .get_result(self.connection)
            .expect("Error saving new input")
    }
}
