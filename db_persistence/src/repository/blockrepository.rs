use super::Repository;
use domain::Block;
use domain::NewBlock;
use schema::blocks;
use schema::blocks::dsl::*;
use diesel;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::sql_query;

const GENESIS_BLOCK_HASH: &'static str =
  "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";

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

  /// Read all blocks, ordered by id.
  pub fn read_all(&self) -> Vec<Block> {
    // TODO Return error instead of panicking.
    blocks.order(id).load::<Block>(self.connection).unwrap()
  }

  /// Calculate the block height for all blocks.
  ///
  /// Returns the number of affected rows.
  pub fn calculate_block_height(&self) -> usize {
    let query = format!(
      r"
      WITH RECURSIVE brec AS (
        SELECT b0.hash, 0 AS height
          FROM blocks b0
          WHERE b0.hash = E'\\x{}'
        UNION ALL
        SELECT b1.hash, (brec.height + 1) AS height
          FROM blocks b1 JOIN brec ON b1.previous_block_hash = brec.hash
        ) UPDATE blocks SET height = brec.height
          FROM brec
          WHERE brec.hash = blocks.hash;
    ",
      GENESIS_BLOCK_HASH
    );

    // TODO Return error instead of panicking.
    sql_query(query).execute(self.connection).unwrap()
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
