use blk_file_reader;
use diesel::{self, pg::PgConnection, RunQueryDsl};
use domain::Input;
use schema::inputs;
use std::result::Result;

#[derive(Insertable)]
#[table_name = "inputs"]
pub struct NewInput {
    pub sequence_number: i32,
    pub previous_tx_hash: Vec<u8>,
    pub previous_tx_output_index: i32,
    pub script: Vec<u8>,
    pub transaction_id: i64,
}

impl NewInput {
    pub fn new(input: &blk_file_reader::Input, transaction_id: i64) -> NewInput {
        NewInput {
            sequence_number: input.sequence_number as i32,
            // TODO Avoid copy.
            previous_tx_hash: input.previous_tx_hash.0.to_vec(),
            previous_tx_output_index: input.previous_tx_output_index as i32,
            // TODO Avoid copy.
            script: input.script.to_vec(),
            transaction_id,
        }
    }

    pub fn save(&self, db_connection: &PgConnection) -> Result<Input, diesel::result::Error> {
        diesel::insert_into(inputs::table)
            .values(self)
            .get_result(db_connection)
    }
}
