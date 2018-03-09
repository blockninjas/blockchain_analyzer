extern crate blk_file_reader;
extern crate db_persistence;
extern crate diesel;
#[macro_use]
extern crate log;
extern crate rayon;

mod blkfileimporter;

use rayon::prelude::*;
use diesel::prelude::*;
use blk_file_reader::list_blk_files;
use blkfileimporter::BlkFileImporter;
use db_persistence::establish_connection;

pub fn import_blk_files(path: &str, database_url: &str) -> std::io::Result<()> {
  let blk_files = list_blk_files(path)?;

  // TODO Make number of threads configurable.
  blk_files
    .par_iter()
    .map(|blk_file| import_blk_file(blk_file, database_url))
    .reduce_with(|r1, r2| if r1.is_err() { r1 } else { r2 })
    .unwrap_or(Ok(()))
}

fn import_blk_file(blk_file_path: &str, database_url: &str) -> std::io::Result<()> {
  info!("Parse {}", blk_file_path);
  let db_connection = establish_connection(database_url);
  let _ = db_connection
    .transaction::<(), diesel::result::Error, _>(|| {
      let blk_file_importer = BlkFileImporter::new(&db_connection);
      blk_file_importer.import(blk_file_path)
    })
    .unwrap();
  Ok(())
}
