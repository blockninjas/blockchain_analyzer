//! # Genesis Test
//!
//! Verifies that the genesis block is read correctly.

extern crate blk_file_reader;
extern crate data_encoding;

use blk_file_reader::read_blocks;
use data_encoding::HEXLOWER;

const PATH_TO_BLK_FILE_0: &'static str = "../sample-blk-files/blk00000.dat";

// TODO Split up into multiple tests to improve readability.
#[test]
pub fn can_read_genesis_block() {
    // given
    let mut blocks = read_blocks(PATH_TO_BLK_FILE_0).unwrap();
    // when
    let block = blocks.next().unwrap().unwrap();
    // then
    assert_eq!(block.version, 1);
    assert_eq!(
        HEXLOWER.encode(&block.hash.0),
        "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
    );
    assert_eq!(
        HEXLOWER.encode(&block.previous_block_hash.0),
        "0000000000000000000000000000000000000000000000000000000000000000"
    );
    assert_eq!(
        HEXLOWER.encode(&block.merkle_root.0),
        "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"
    );
    assert_eq!(block.creation_time, 1231006505);
    assert_eq!(block.bits, 486604799);
    assert_eq!(block.nonce, 2083236893);
    assert_eq!(block.transactions.len(), 1);
    assert_eq!(
        HEXLOWER.encode(&block.transactions[0].tx_hash.0),
        "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"
    );
    assert_eq!(block.transactions[0].version, 1);
    assert_eq!(block.transactions[0].lock_time, 0);
    assert_eq!(block.transactions[0].inputs.len(), 1);
    assert_eq!(block.transactions[0].inputs[0].sequence_number, 4294967295);
    assert_eq!(
        HEXLOWER.encode(&block.transactions[0].inputs[0].previous_tx_hash.0),
        "0000000000000000000000000000000000000000000000000000000000000000"
    );
    assert_eq!(
        block.transactions[0].inputs[0].previous_tx_output_index,
        4294967295
    );
    assert_eq!(block.transactions[0].outputs.len(), 1);
    assert_eq!(block.transactions[0].outputs[0].index, 0);
    assert_eq!(block.transactions[0].outputs[0].value, 5_000_000_000);
    assert_eq!(block.transactions[0].weight, 816);

    let address = block.transactions[0].outputs[0].address.as_ref().unwrap();
    assert!(block.transactions[0].outputs[0].address.is_some());
    assert_eq!(
        address.hash.to_vec(),
        HEXLOWER
            .decode(b"62e907b15cbf27d5425399ebf6f0fb50ebb88f18")
            .unwrap()
    );
    assert_eq!(address.base58check, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
}
