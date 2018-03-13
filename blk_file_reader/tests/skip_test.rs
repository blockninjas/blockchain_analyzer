//! # Skip Test
//!
//! A collection of tests for `BlockReader::skip()`.

extern crate blk_file_reader;

use blk_file_reader::BlockRead;
use blk_file_reader::BlockReader;

const PATH_TO_BLK_FILE_0: &'static str = "sample_blk_files/blk00000.dat";

#[test]
fn skip_0_skips_no_blocks() {
  // given
  let mut block_reader = BlockReader::from_blk_file(PATH_TO_BLK_FILE_0);
  // when skip is called with `0`
  block_reader.skip(0).unwrap();
  // then `read()` returns the genesis block
  let block = block_reader.read().unwrap();
  assert_eq!(
    block.previous_block_hash.to_hex_string(),
    "0000000000000000000000000000000000000000000000000000000000000000"
  );
}

#[test]
fn skip_1_skips_one_block() {
  // given
  let mut block_reader = BlockReader::from_blk_file(PATH_TO_BLK_FILE_0);
  // when skip is called with `1`
  block_reader.skip(1).unwrap();
  // then `read()` returns the successor of the genesis block
  let block = block_reader.read().unwrap();
  assert_eq!(
    block.previous_block_hash.to_hex_string().to_lowercase(),
    "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
  );
}
