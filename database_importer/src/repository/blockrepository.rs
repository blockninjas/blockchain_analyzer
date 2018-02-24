use super::Repository;
use ::domain::Block;
use ::domain::NewBlock;
use ::schema::blocks;
use diesel;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;

pub struct BlockRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> BlockRepository<'a> {
    pub fn new(connection: &'a PgConnection) -> BlockRepository<'a> {
        BlockRepository {
            connection
        }
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
