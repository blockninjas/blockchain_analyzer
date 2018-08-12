use super::BlkFile;
use diesel::{self, pg::PgConnection, RunQueryDsl};
use schema::blk_files;
use std::result::Result;

#[derive(Insertable)]
#[table_name = "blk_files"]
pub struct NewBlkFile {
    pub name: String,
    pub number_of_blocks: i32,
}

impl NewBlkFile {
    pub fn save(&self, db_connection: &PgConnection) -> Result<BlkFile, diesel::result::Error> {
        // TODO Return error instead of panicking.
        diesel::insert_into(blk_files::table)
            .values(self)
            .get_result(db_connection)
    }
}
