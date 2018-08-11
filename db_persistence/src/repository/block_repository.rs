use diesel::{self, dsl::max, pg::PgConnection, QueryDsl, RunQueryDsl};
use domain::{Block, NewBlock};
use schema::blocks;
use schema::blocks::dsl::*;
use std::result::Result;

pub struct BlockRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> BlockRepository<'a> {
    pub fn new(connection: &'a PgConnection) -> BlockRepository<'a> {
        BlockRepository { connection }
    }

    pub fn count(&self) -> Result<i64, diesel::result::Error> {
        // TODO Return error instead of panicking.
        blocks.count().get_result(self.connection)
    }

    pub fn max_height(&self) -> Result<Option<i32>, diesel::result::Error> {
        // TODO Return error instead of panicking.
        blocks.select(max(height)).first(self.connection)
    }

    /// Read all blocks, ordered by id.
    pub fn read_all(&self) -> Result<Vec<Block>, diesel::result::Error> {
        // TODO Return error instead of panicking.
        blocks.order(id).load::<Block>(self.connection)
    }

    pub fn save(&self, new_block: &NewBlock) -> Result<Block, diesel::result::Error> {
        diesel::insert_into(blocks::table)
            .values(new_block)
            .get_result(self.connection)
    }
}
