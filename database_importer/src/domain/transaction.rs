use super::super::schema::transactions;

#[derive(Insertable)]
#[table_name="transactions"]
pub struct Transaction {
    pub id: i32,
    pub hash: Vec<u8>,
    pub version: i32,
    pub lock_time: i32,
    pub creation_time: i32,
    pub block_id: i32,
}
