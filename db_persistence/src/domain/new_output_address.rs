use blk_file_reader;
use diesel::{self, pg::PgConnection, RunQueryDsl};
use domain::OutputAddress;
use schema::output_addresses;
use std::result::Result;

#[derive(Insertable)]
#[table_name = "output_addresses"]
pub struct NewOutputAddress {
    pub output_id: i64,
    pub hash: Vec<u8>,
    pub base58check: String,
}

impl NewOutputAddress {
    pub fn new(address: &blk_file_reader::Address, output_id: i64) -> NewOutputAddress {
        NewOutputAddress {
            output_id,
            hash: address.hash.to_vec(),
            base58check: address.base58check.clone(),
        }
    }

    pub fn save(
        &self,
        db_connection: &PgConnection,
    ) -> Result<OutputAddress, diesel::result::Error> {
        diesel::insert_into(output_addresses::table)
            .values(self)
            .get_result(db_connection)
    }
}
