use diesel;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::pg::PgConnection;
use domain::BlkFile;
use domain::NewBlkFile;
use schema::blk_files;
use schema::blk_files::dsl::*;

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

  pub fn read_all(&self) -> Vec<BlkFile> {
    blk_files
      .load::<BlkFile>(self.connection)
      .expect("Error loading blk files")
  }

  pub fn read_all_names(&self) -> Vec<String> {
    blk_files
      .select(name)
      .load::<String>(self.connection)
      .unwrap()
  }

  pub fn save(&self, new_blk_file: &NewBlkFile) -> BlkFile {
    // TODO Return error instead of panicking.
    diesel::insert_into(blk_files::table)
      .values(new_blk_file)
      .get_result(self.connection)
      .expect("Error saving new blk file")
  }
}
