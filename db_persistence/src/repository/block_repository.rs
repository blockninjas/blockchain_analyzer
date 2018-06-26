use diesel::{self, dsl::max, pg::PgConnection, QueryDsl, RunQueryDsl};
use domain::{Block, NewBlock};
use schema::blocks;
use schema::blocks::dsl::*;

pub struct BlockRepository<'a> {
  connection: &'a PgConnection,
}

impl<'a> BlockRepository<'a> {
  pub fn new(connection: &'a PgConnection) -> BlockRepository<'a> {
    BlockRepository { connection }
  }

  pub fn count(&self) -> i64 {
    // TODO Return error instead of panicking.
    blocks.count().get_result(self.connection).unwrap()
  }

  pub fn max_height(&self) -> Option<i32> {
    // TODO Return error instead of panicking.
    blocks.select(max(height)).first(self.connection).unwrap()
  }

  /// Read all blocks, ordered by id.
  pub fn read_all(&self) -> Vec<Block> {
    // TODO Return error instead of panicking.
    blocks.order(id).load::<Block>(self.connection).unwrap()
  }

  pub fn save(&self, new_block: &NewBlock) -> Block {
    diesel::insert_into(blocks::table)
      .values(new_block)
      .get_result(self.connection)
      .expect("Error saving new block")
  }
}
