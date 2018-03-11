//! # Block Reader Test
//!
//! Integration tests for `BlockReader`.

extern crate blk_file_reader;

use blk_file_reader::BlockReader;

const PATH_TO_BLK_FILE_0: &'static str = "sample_blk_files/blk00000.dat";

#[test]
pub fn can_read_genesis_block() {
  // given
  let mut block_reader = BlockReader::new(PATH_TO_BLK_FILE_0);
  // when
  let block = block_reader.read().unwrap();
  // then
  assert_eq!(block.version, 1);
  assert_eq!(
    block.hash.to_hex_string().to_lowercase(),
    String::from("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f")
  );
  assert_eq!(
    block.previous_block_hash.to_hex_string().to_lowercase(),
    String::from("0000000000000000000000000000000000000000000000000000000000000000")
  );
  assert_eq!(
    block.merkle_root.to_hex_string().to_lowercase(),
    String::from("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b")
  );
  assert_eq!(block.creation_time, 1231006505);
  assert_eq!(block.bits, 486604799);
  assert_eq!(block.nonce, 2083236893);
  assert_eq!(block.transactions.len(), 1);
}
