//! # BlkFileRepository Test

extern crate db_persistence;
extern crate diesel;

use db_persistence::domain::NewBlkFile;
use db_persistence::repository::BlkFileRepository;
use diesel::prelude::*;
use diesel::result::Error;

// TODO Make database URL configurable.
const TEST_DATABASE_URL: &'static str =
  "postgres://postgres:test@127.0.0.1:5432/bitcoin_blockchain";

#[test]
pub fn can_save_blk_files() {
  // Given
  let new_blk_file = NewBlkFile {
    name: String::from("blk00000.dat"),
    number_of_blocks: 42,
  };
  let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();

  db_connection.test_transaction::<_, Error, _>(|| {
    // When
    let blk_file_repository = BlkFileRepository::new(&db_connection);
    let saved_blk_file = blk_file_repository.save(&new_blk_file)?;

    // Then
    assert_eq!(saved_blk_file.name, new_blk_file.name);
    assert_eq!(
      saved_blk_file.number_of_blocks,
      new_blk_file.number_of_blocks
    );
    Ok(())
  });
}

#[test]
pub fn can_read_all_saved_blk_files() {
  // Given
  let new_blk_file1 = NewBlkFile {
    name: String::from("blk00000.dat"),
    number_of_blocks: 42,
  };
  let new_blk_file2 = NewBlkFile {
    name: String::from("blk00001.dat"),
    number_of_blocks: 43,
  };
  let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();

  db_connection.test_transaction::<_, Error, _>(|| {
    // When
    let blk_file_repository = BlkFileRepository::new(&db_connection);
    let _ = blk_file_repository.save(&new_blk_file1)?;
    let _ = blk_file_repository.save(&new_blk_file2)?;
    let blk_files = blk_file_repository.read_all()?;

    // Then
    assert_eq!(blk_files.len(), 2);
    Ok(())
  });
}

#[test]
pub fn can_read_all_saved_blk_file_names() {
  // Given
  let new_blk_file1 = NewBlkFile {
    name: String::from("blk00000.dat"),
    number_of_blocks: 42,
  };
  let new_blk_file2 = NewBlkFile {
    name: String::from("blk00001.dat"),
    number_of_blocks: 43,
  };
  let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();

  db_connection.test_transaction::<_, Error, _>(|| {
    // When
    let blk_file_repository = BlkFileRepository::new(&db_connection);
    let _ = blk_file_repository.save(&new_blk_file1)?;
    let _ = blk_file_repository.save(&new_blk_file2)?;
    let blk_file_names = blk_file_repository.read_all_names()?;

    // Then
    assert_eq!(blk_file_names.len(), 2);
    assert_eq!(blk_file_names[0], "blk00000.dat");
    assert_eq!(blk_file_names[1], "blk00001.dat");
    Ok(())
  });
}
