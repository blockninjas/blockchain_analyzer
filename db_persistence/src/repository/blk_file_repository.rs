use diesel::{
  self, pg::PgConnection, ExpressionMethods, OptionalExtension, QueryDsl,
  RunQueryDsl,
};
use domain::BlkFile;
use domain::NewBlkFile;
use schema::blk_files;
use schema::blk_files::dsl::*;
use std::result::Result;

pub struct BlkFileRepository<'a> {
  connection: &'a PgConnection,
}

impl<'a> BlkFileRepository<'a> {
  pub fn new(connection: &'a PgConnection) -> BlkFileRepository<'a> {
    BlkFileRepository { connection }
  }

  pub fn read_latest_blk_file(
    &self,
  ) -> Result<Option<BlkFile>, diesel::result::Error> {
    blk_files
      .order(name.desc())
      .first(self.connection)
      .optional()
  }

  pub fn count(&self) -> Result<i64, diesel::result::Error> {
    blk_files::table.count().get_result(self.connection)
  }

  pub fn read_all(&self) -> Result<Vec<BlkFile>, diesel::result::Error> {
    blk_files.load::<BlkFile>(self.connection)
  }

  pub fn read_all_names(&self) -> Result<Vec<String>, diesel::result::Error> {
    blk_files.select(name).load::<String>(self.connection)
  }

  pub fn save(
    &self,
    new_blk_file: &NewBlkFile,
  ) -> Result<BlkFile, diesel::result::Error> {
    // TODO Return error instead of panicking.
    diesel::insert_into(blk_files::table)
      .values(new_blk_file)
      .get_result(self.connection)
  }
}
