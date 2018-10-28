use diesel::{self, dsl::max, pg::PgConnection, QueryDsl, RunQueryDsl};
use schema::blocks::dsl::*;
use std::result::Result;

#[derive(Queryable)]
pub struct Block {
    pub id: i64,
    pub hash: Vec<u8>,
    pub version: i32,
    pub previous_block_hash: Vec<u8>,
    pub merkle_root: Vec<u8>,
    pub creation_time: i32,
    pub bits: i32,
    pub nonce: i32,
    pub height: Option<i32>,
    pub blk_file_id: i64,
}

impl Block {
    pub fn count(db_connection: &PgConnection) -> Result<i64, diesel::result::Error> {
        // TODO Return error instead of panicking.
        blocks.count().get_result(db_connection)
    }

    pub fn max_height(db_connection: &PgConnection) -> Result<Option<i32>, diesel::result::Error> {
        // TODO Return error instead of panicking.
        blocks.select(max(height)).first(db_connection)
    }

    /// Read all blocks, ordered by id.
    pub fn read_all(db_connection: &PgConnection) -> Result<Vec<Block>, diesel::result::Error> {
        // TODO Return error instead of panicking.
        blocks.order(id).load::<Block>(db_connection)
    }
}
