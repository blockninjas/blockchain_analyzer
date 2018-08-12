use blk_file_reader;
use diesel::{self, pg::PgConnection, RunQueryDsl};
use domain::Output;
use schema::outputs;
use std::result::Result;

#[derive(Insertable)]
#[table_name = "outputs"]
pub struct NewOutput {
    pub output_index: i32,
    pub value: i64,
    pub script: Vec<u8>,
    pub transaction_id: i64,
}

impl NewOutput {
    pub fn new(output: &blk_file_reader::Output, transaction_id: i64) -> NewOutput {
        NewOutput {
            output_index: output.index as i32,
            value: output.value as i64,
            // TODO Avoid copy.
            script: output.script.to_vec(),
            transaction_id,
        }
    }

    pub fn save(&self, db_connection: &PgConnection) -> Result<Output, diesel::result::Error> {
        diesel::insert_into(outputs::table)
            .values(self)
            .get_result(db_connection)
    }
}
