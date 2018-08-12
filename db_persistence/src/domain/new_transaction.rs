use blk_file_reader;
use diesel::{self, pg::PgConnection, RunQueryDsl};
use domain::Transaction;
use schema::transactions;
use std::result::Result;

#[derive(Insertable, Default)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub hash: Vec<u8>,
    pub version: i32,
    pub lock_time: i32,
    pub size_in_bytes: i32,
    pub weight: i32,
    pub block_id: i64,
}

impl NewTransaction {
    pub fn new(transaction: &blk_file_reader::Transaction, block_id: i64) -> NewTransaction {
        NewTransaction {
            hash: transaction.tx_hash.0.to_vec(),
            version: transaction.version as i32,
            lock_time: transaction.lock_time as i32,
            size_in_bytes: transaction.size_in_bytes as i32,
            weight: transaction.weight as i32,
            block_id,
        }
    }

    pub fn save(&self, db_connection: &PgConnection) -> Result<Transaction, diesel::result::Error> {
        diesel::insert_into(transactions::table)
            .values(self)
            .get_result(db_connection)
    }
}
