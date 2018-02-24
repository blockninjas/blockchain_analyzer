use ::schema::transactions;
use blk_file_reader;

#[derive(Insertable)]
#[table_name="transactions"]
pub struct NewTransaction {
    pub hash: Vec<u8>,
    pub version: i32,
    pub lock_time: i32,
    pub creation_time: i32,
    pub block_id: i32,
}

impl NewTransaction {
    pub fn new(transaction: &blk_file_reader::Transaction, block_id: i32)
            -> NewTransaction {
        NewTransaction {
            hash: transaction.tx_hash.0.to_vec(),
            version: transaction.version as i32,
            lock_time: transaction.lock_time as i32,
            creation_time: transaction.creation_time as i32,
            block_id,
        }
    }
}
