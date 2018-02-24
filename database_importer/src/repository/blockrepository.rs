use ::domain::Block;
use ::schema::blocks;
use ::schema::blocks::dsl::*;
use super::Repository;
use diesel;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;

pub struct BlockRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> BlockRepository<'a> {
    fn new(connection: &'a PgConnection) -> BlockRepository<'a> {
        BlockRepository {
            connection
        }
    }
}

impl<'a> Repository<Block> for BlockRepository<'a> {
    fn save(&self, block: &Block) {
        diesel::insert_into(blocks::table)
            .values(block)
            .execute(self.connection)
            .expect("Error saving new block");
    }
}
