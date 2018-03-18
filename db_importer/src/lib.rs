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
use db_persistence::repository::BlkFileRepository;

pub fn import_blk_files(path: &str, database_url: &str) -> std::io::Result<()> {
  let blk_files = list_blk_files(path)?;

  // TODO Return error instead of panicking.
  let db_connection = PgConnection::establish(database_url).unwrap();
  let blk_file_repository = BlkFileRepository::new(&db_connection);

  // TODO Fix possibly trucating cast.
  let number_of_imported_blk_files = blk_file_repository.count() as usize;

  // TODO Ensure that all blk files with an index smaller than
  // `number_of_imported_blk_files` have sucessfully been imported, otherwise
  // retry importing them.

  let not_yet_imported_blk_files = &blk_files[number_of_imported_blk_files..];

  // Do not import the latest 2 blk files to be able to ignore blockchain
  // reorganizations.
  // TODO Make this configurible.
  let number_of_files_to_skip_at_end = 2;
  let number_of_blk_files_to_import =
    not_yet_imported_blk_files.len() - number_of_files_to_skip_at_end;

  let blk_files_to_import =
    &not_yet_imported_blk_files[..number_of_blk_files_to_import];

  // TODO Make number of threads configurable.
  blk_files_to_import
    .par_iter()
    .map(|blk_file| import_blk_file(blk_file, database_url))
    .reduce_with(|r1, r2| if r1.is_err() { r1 } else { r2 })
    .unwrap_or(Ok(()))
}

fn import_blk_file(
  blk_file_path: &str,
  database_url: &str,
) -> std::io::Result<()> {
  info!("Parse {}", blk_file_path);
  // TODO Return error instead of panicking.
  let db_connection = PgConnection::establish(database_url).unwrap();
  let _ = db_connection
    .transaction::<(), diesel::result::Error, _>(|| {
      let blk_file_importer = BlkFileImporter::new(&db_connection);
      blk_file_importer.import(blk_file_path)
    })
    .unwrap();
  Ok(())
}
