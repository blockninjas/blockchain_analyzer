//! # SegWit Test
//!
//! Verifies that blocks that contain witness scripts are read correctly.

extern crate blk_file_reader;
extern crate data_encoding;

use blk_file_reader::read_blocks;
use data_encoding::HEXLOWER;

const PATH_TO_SEG_WIT_BLK_FILE: &'static str = "../sample-blk-files/blk01208.dat";

// TODO Split up into multiple tests to improve readability.
#[test]
pub fn can_read_seg_wit_block() {
    // given
    let mut blocks = read_blocks(PATH_TO_SEG_WIT_BLK_FILE).unwrap();
    // when
    let block = blocks.next().unwrap().unwrap();
    // then
    assert_eq!(block.version, 0x20000000);
    assert_eq!(
        HEXLOWER.encode(&block.hash.0),
        "00000000000000000014480b3727b23327504bfb49192205c0872cd61ee69d02"
    );
    assert_eq!(
        HEXLOWER.encode(&block.previous_block_hash.0),
        "00000000000000000005071a2d7506843bdc5d10c2ed93f0aa1fded1b3699379"
    );
    assert_eq!(
        HEXLOWER.encode(&block.merkle_root.0),
        "5c86d1a2f183600953460f4082e76fa704b5e12eebcb57767bed6439d07d9a80"
    );
    assert_eq!(block.creation_time, 1520850157);
    assert_eq!(block.bits, 391481763);
    assert_eq!(block.nonce, 2287239028);
    assert_eq!(block.transactions.len(), 1276);
    assert_eq!(
        HEXLOWER.encode(&block.transactions[19].tx_hash.0),
        "ddd8817c80ed3c7c03eb503eb1071f16ffead123626a28d408aefcc06f7f2e14"
    );
    assert_eq!(
        HEXLOWER.encode(&block.transactions[19].witness_hash.0),
        "697abd7cd319e30a83e887aad86a91ce06eab915025ea83d694cbab708aded31"
    );
    assert_eq!(block.transactions[19].weight, 766);
}
