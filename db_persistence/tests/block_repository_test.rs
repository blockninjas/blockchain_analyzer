//! # BlockRepository Test

extern crate data_encoding;
extern crate db_persistence;
extern crate diesel;

use data_encoding::HEXLOWER;
use db_persistence::domain::NewBlock;
use db_persistence::repository::BlockRepository;
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
    hash: HEXLOWER
      .decode(
        b"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
      )
      .unwrap(),
    previous_block_hash: HEXLOWER
      .decode(
        b"0000000000000000000000000000000000000000000000000000000000000000",
      )
      .unwrap(),
    merkle_root: HEXLOWER
      .decode(
        b"4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b",
      )
      .unwrap(),
    creation_time: 1231006505,
    nonce: 2083236893,
  };

  db_connection.test_transaction::<_, Error, _>(|| {
    // When
    let block_repository = BlockRepository::new(&db_connection);
    let saved_block = block_repository.save(&new_block).unwrap();

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
    let saved_block = block_repository.save(&new_block).unwrap();

    // Then
    assert_eq!(saved_block.version as u32, u32::max_value() - 1);
    Ok(())
  });
}
