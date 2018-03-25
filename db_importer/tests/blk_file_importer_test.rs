//! # BlkFileImporter Test

extern crate blk_file_reader;
extern crate db_importer;
extern crate db_persistence;
extern crate diesel;

use db_importer::BlkFileImporter;
use db_persistence::repository::{BlkFileRepository, BlockRepository};
use diesel::prelude::*;
use diesel::result::Error;
use blk_file_reader::read_blocks;

const TEST_DATABASE_URL: &'static str =
  "postgres://postgres:test@127.0.0.1:5432/bitcoin_blockchain";

#[test]
pub fn genesis_block_is_imported_correctly() {
  // Given
  let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();
  let blk_file_importer = BlkFileImporter::new(&db_connection);
  let blocks =
    read_blocks("../blk_file_reader/sample_blk_files/blk00000.dat").unwrap();
  let blocks = blocks.take(1);

  db_connection.test_transaction::<_, Error, _>(|| {
    // When
    let _ = blk_file_importer
      .import("blk00000.dat", blocks.into_iter())
      .unwrap();

    // Then
    let block_repository = BlockRepository::new(&db_connection);
    let imported_blocks = block_repository.read_all();
    assert_eq!(imported_blocks.len(), 1);

    let genesis_block = &imported_blocks[0];
    assert_eq!(genesis_block.version, 1);
    assert_eq!(
      genesis_block.hash,
      vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0xd6, 0x68, 0x9c, 0x08, 0x5a, 0xe1,
        0x65, 0x83, 0x1e, 0x93, 0x4f, 0xf7, 0x63, 0xae, 0x46, 0xa2, 0xa6, 0xc1,
        0x72, 0xb3, 0xf1, 0xb6, 0x0a, 0x8c, 0xe2, 0x6f,
      ]
    );
    assert!(genesis_block.previous_block_hash.iter().all(|&b| b == 0));
    assert_eq!(
      genesis_block.merkle_root,
      vec![
        0x4a, 0x5e, 0x1e, 0x4b, 0xaa, 0xb8, 0x9f, 0x3a, 0x32, 0x51, 0x8a, 0x88,
        0xc3, 0x1b, 0xc8, 0x7f, 0x61, 0x8f, 0x76, 0x67, 0x3e, 0x2c, 0xc7, 0x7a,
        0xb2, 0x12, 0x7b, 0x7a, 0xfd, 0xed, 0xa3, 0x3b,
      ]
    );
    assert_eq!(genesis_block.creation_time, 1231006505);
    assert_eq!(genesis_block.nonce, 2083236893);

    Ok(())
  });
}

#[test]
pub fn imports_all_provided_blocks() {
  // Given
  let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();
  let blk_file_importer = BlkFileImporter::new(&db_connection);
  let blocks =
    read_blocks("../blk_file_reader/sample_blk_files/blk00000.dat").unwrap();
  let blocks = blocks.take(5);

  db_connection.test_transaction::<_, Error, _>(|| {
    // When
    let _ = blk_file_importer
      .import("blk00000.dat", blocks.into_iter())
      .unwrap();

    // Then
    let block_repository = BlockRepository::new(&db_connection);
    assert_eq!(block_repository.count(), 5);
    let blk_file_repository = BlkFileRepository::new(&db_connection);
    let blk_files = blk_file_repository.read_all();
    assert_eq!(blk_files.len(), 1);
    let blk_file = &blk_files[0];
    assert_eq!(blk_file.name, "blk00000.dat");
    assert_eq!(blk_file.number_of_blocks, 5);

    Ok(())
  });
}

#[test]
pub fn can_import_empty_blk_file() {
  // Given
  let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();
  let blk_file_importer = BlkFileImporter::new(&db_connection);
  let blocks = Vec::<std::io::Result<blk_file_reader::Block>>::new();

  db_connection.test_transaction::<_, Error, _>(|| {
    // When
    let _ = blk_file_importer
      .import("blk12345.dat", blocks.into_iter())
      .unwrap();

    // Then
    let block_repository = BlockRepository::new(&db_connection);
    assert_eq!(block_repository.count(), 0);
    let blk_file_repository = BlkFileRepository::new(&db_connection);
    let blk_files = blk_file_repository.read_all();
    assert_eq!(blk_files.len(), 1);
    let blk_file = &blk_files[0];
    assert_eq!(blk_file.name, "blk12345.dat");
    assert_eq!(blk_file.number_of_blocks, 0);

    Ok(())
  });
}
