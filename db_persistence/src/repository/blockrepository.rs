use super::Repository;
use domain::Block;
use domain::NewBlock;
use schema::blocks;
use schema::blocks::dsl::*;
use diesel;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;
use diesel::QueryDsl;

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

  pub fn read_all(&self) -> Vec<Block> {
    // TODO Return error instead of panicking.
    blocks.load::<Block>(self.connection).unwrap()
  }
}

impl<'a> Repository for BlockRepository<'a> {
  type NewEntity = NewBlock;
  type Entity = Block;

  fn save(&self, new_block: &NewBlock) -> Block {
    diesel::insert_into(blocks::table)
      .values(new_block)
      .get_result(self.connection)
      .expect("Error saving new block")
  }
}
