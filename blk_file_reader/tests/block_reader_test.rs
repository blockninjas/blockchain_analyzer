//! # Block Reader Test
//!
//! Integration tests for `BlockReader`.

extern crate blk_file_reader;

use blk_file_reader::BlockRead;
use blk_file_reader::BlockReader;

const PATH_TO_BLK_FILE_0: &'static str = "sample_blk_files/blk00000.dat";

// TODO Split up into multiple tests to improve readability.
#[test]
pub fn can_read_genesis_block() {
  // given
  let mut block_reader = BlockReader::from_blk_file(PATH_TO_BLK_FILE_0);
  // when
  let block = block_reader.read().unwrap();
  // then
  assert_eq!(block.version, 1);
  assert_eq!(
    block.hash.to_hex_string().to_lowercase(),
    "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
  );
  assert_eq!(
    block.previous_block_hash.to_hex_string().to_lowercase(),
    "0000000000000000000000000000000000000000000000000000000000000000"
  );
  assert_eq!(
    block.merkle_root.to_hex_string().to_lowercase(),
    "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"
  );
  assert_eq!(block.creation_time, 1231006505);
  assert_eq!(block.bits, 486604799);
  assert_eq!(block.nonce, 2083236893);
  assert_eq!(block.transactions.len(), 1);
  assert_eq!(
    block.transactions[0].tx_hash.to_hex_string().to_lowercase(),
    "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"
  );
  assert_eq!(block.transactions[0].version, 1);
  assert_eq!(block.transactions[0].lock_time, 0);
  assert_eq!(block.transactions[0].inputs.len(), 1);
  assert_eq!(block.transactions[0].inputs[0].sequence_number, 4294967295);
  assert_eq!(
    block.transactions[0].inputs[0]
      .previous_tx_hash
      .to_hex_string()
      .to_lowercase(),
    "0000000000000000000000000000000000000000000000000000000000000000"
  );
  assert_eq!(
    block.transactions[0].inputs[0].previous_tx_output_index,
    4294967295
  );
  assert_eq!(block.transactions[0].outputs.len(), 1);
  assert_eq!(block.transactions[0].outputs[0].index, 0);
  assert_eq!(block.transactions[0].outputs[0].value, 5_000_000_000);
  assert_eq!(block.transactions[0].outputs[0].addresses.len(), 1);
  assert_eq!(
    block.transactions[0].outputs[0].addresses[0].base58_string,
    "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
  );
}
