//! # SegWit Test
//!
//! Verifies that `BlockReader` is able to process blocks that contain witness
//! scripts.

extern crate blk_file_reader;

use blk_file_reader::BlockRead;
use blk_file_reader::BlockReader;

const PATH_TO_SEG_WIT_BLK_FILE: &'static str = "sample_blk_files/blk01208.dat";

// TODO Split up into multiple tests to improve readability.
#[test]
pub fn can_read_seg_wit_block() {
  // given
  let mut block_reader = BlockReader::from_blk_file(PATH_TO_SEG_WIT_BLK_FILE);
  // when
  let block = block_reader.read().unwrap();
  // then
  assert_eq!(block.version, 0x20000000);
  assert_eq!(
    block.hash.to_hex_string().to_lowercase(),
    "00000000000000000014480b3727b23327504bfb49192205c0872cd61ee69d02"
  );
  assert_eq!(
    block.previous_block_hash.to_hex_string().to_lowercase(),
    "00000000000000000005071a2d7506843bdc5d10c2ed93f0aa1fded1b3699379"
  );
  assert_eq!(
    block.merkle_root.to_hex_string().to_lowercase(),
    "5c86d1a2f183600953460f4082e76fa704b5e12eebcb57767bed6439d07d9a80"
  );
  assert_eq!(block.creation_time, 1520850157);
  assert_eq!(block.bits, 391481763);
  assert_eq!(block.nonce, 2287239028);
  assert_eq!(block.transactions.len(), 1276);
}
