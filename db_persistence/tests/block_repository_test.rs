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
    let saved_block = block_repository.save(&new_block);

    // Then
    assert_eq!(saved_block.version, new_block.version);
    assert_eq!(saved_block.hash, new_block.hash);
    assert_eq!(
      saved_block.previous_block_hash,
      new_block.previous_block_hash
    );
    assert_eq!(saved_block.merkle_root, new_block.merkle_root);
    assert_eq!(
      saved_block.creation_time,
      new_block.creation_time
    );
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

#[test]
pub fn can_calculate_block_height() {
  // Given
  let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();

  let new_block0 = NewBlock {
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
    merkle_root: vec![],
    creation_time: 0,
    nonce: 0,
  };

  let new_block1 = NewBlock {
    version: 1,
    hash: HEXLOWER
      .decode(
        b"00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048",
      )
      .unwrap(),
    previous_block_hash: HEXLOWER
      .decode(
        b"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
      )
      .unwrap(),
    merkle_root: vec![],
    creation_time: 0,
    nonce: 0,
  };

  let new_block2 = NewBlock {
    version: 1,
    hash: HEXLOWER
      .decode(
        b"000000006a625f06636b8bb6ac7b960a8d03705d1ace08b1a19da3fdcc99ddbd",
      )
      .unwrap(),
    previous_block_hash: HEXLOWER
      .decode(
        b"00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048",
      )
      .unwrap(),
    merkle_root: vec![],
    creation_time: 0,
    nonce: 0,
  };

  db_connection.test_transaction::<_, Error, _>(|| {
    // When
    let block_repository = BlockRepository::new(&db_connection);
    let _ = block_repository.save(&new_block0);
    let _ = block_repository.save(&new_block1);
    let _ = block_repository.save(&new_block2);
    let affected_blocks = block_repository.calculate_block_height();

    // Then
    assert_eq!(affected_blocks, 3);
    let blocks = block_repository.read_all();
    assert_eq!(blocks.len(), 3);
    assert_eq!(blocks[0].height, Some(0));
    assert_eq!(blocks[1].height, Some(1));
    assert_eq!(blocks[2].height, Some(2));
    Ok(())
  });
}
