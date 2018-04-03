//! # BlockRepository Test

extern crate db_persistence;
extern crate diesel;

use db_persistence::domain::NewBlock;
use db_persistence::repository::{BlockRepository, Repository};
use diesel::prelude::*;
use diesel::result::Error;

// TODO Make database URL configurable.
const TEST_DATABASE_URL: &'static str =
  "postgres://postgres:test@127.0.0.1:5432/bitcoin_blockchain";

#[test]
pub fn can_save_block() {
  // Given
  let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();
  let new_block = NewBlock {
    version: 1,
    hash: vec![
      0x19, 0xd6, 0x68, 0x9c, 0x08, 0x5a, 0xe1, 0x65, 0x83, 0x1e, 0x93, 0x4f,
      0xf7, 0x63, 0xae, 0x46, 0xa2, 0xa6, 0xc1, 0x72, 0xb3, 0xf1, 0xb6, 0x0a,
      0x8c, 0xe2, 0x6f,
    ],
    previous_block_hash: vec![],
    merkle_root: vec![
      0x4a, 0x5e, 0x1e, 0x4b, 0xaa, 0xb8, 0x9f, 0x3a, 0x32, 0x51, 0x8a, 0x88,
      0xc3, 0x1b, 0xc8, 0x7f, 0x61, 0x8f, 0x76, 0x67, 0x3e, 0x2c, 0xc7, 0x7a,
      0xb2, 0x12, 0x7b, 0x7a, 0xfd, 0xed, 0xa3, 0x3b,
    ],
    creation_time: 1231006505,
    nonce: 2083236893,
  };

  db_connection.test_transaction::<_, Error, _>(|| {
    // When
    let block_repository = BlockRepository::new(&db_connection);
    let saved_block = block_repository.save(&new_block);

    // Then
    assert_eq!(saved_block.version, new_block.version);
    assert_eq!(saved_block.hash, new_block.hash);
    assert_eq!(
      saved_block.previous_block_hash,
      new_block.previous_block_hash
    );
    assert_eq!(saved_block.merkle_root, new_block.merkle_root);
    assert_eq!(saved_block.creation_time, new_block.creation_time);
    assert_eq!(saved_block.nonce, new_block.nonce);
    assert_eq!(saved_block.height, None);
    Ok(())
  });
}

#[test]
pub fn conversions_are_safe() {
  // Given
  let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();
  let new_block = NewBlock {
    version: (u32::max_value() - 1) as i32,
    hash: vec![],
    previous_block_hash: vec![],
    merkle_root: vec![],
    creation_time: 1231006505,
    nonce: 2083236893,
  };

  db_connection.test_transaction::<_, Error, _>(|| {
    // When
    let block_repository = BlockRepository::new(&db_connection);
    let saved_block = block_repository.save(&new_block);

    // Then
    assert_eq!(saved_block.version as u32, u32::max_value() - 1);
    Ok(())
  });
}
