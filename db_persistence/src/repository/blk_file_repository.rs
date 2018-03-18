use super::Repository;
use domain::BlkFile;
use domain::NewBlkFile;
use schema::blk_files;
use diesel;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;
use diesel::QueryDsl;

pub struct BlkFileRepository<'a> {
  connection: &'a PgConnection,
}

impl<'a> BlkFileRepository<'a> {
  pub fn new(connection: &'a PgConnection) -> BlkFileRepository<'a> {
    BlkFileRepository { connection }
  }

  pub fn count(&self) -> i64 {
    blk_files::table
      .count()
      .get_result(self.connection)
      .expect("Could not retrieve count of blk_files.")
  }
}

impl<'a> Repository for BlkFileRepository<'a> {
  type NewEntity = NewBlkFile;
  type Entity = BlkFile;

  fn save(&self, new_blk_file: &NewBlkFile) -> BlkFile {
    diesel::insert_into(blk_files::table)
      .values(new_blk_file)
      .get_result(self.connection)
      .expect("Error saving new blk file")
  }
}
