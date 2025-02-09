extern crate blk_file_reader;
extern crate data_encoding;

use blk_file_reader::read_blocks;
use data_encoding::HEXLOWER;

const PATH_TO_BLK_FILE_0: &'static str = "../sample-blk-files/blk00000.dat";

#[test]
fn skip_0_skips_no_blocks() {
    // given
    let blocks = read_blocks(PATH_TO_BLK_FILE_0).unwrap();
    // when skip is called with `0`
    let mut blocks = blocks.skip(0);
    // then `read()` returns the genesis block
    let block = blocks.next().unwrap().unwrap();
    assert_eq!(
        HEXLOWER.encode(&block.previous_block_hash.0),
        "0000000000000000000000000000000000000000000000000000000000000000"
    );
}

#[test]
fn skip_1_skips_one_block() {
    // given
    let blocks = read_blocks(PATH_TO_BLK_FILE_0).unwrap();
    // when skip is called with `1`
    let mut blocks = blocks.skip(1);
    // then `read()` returns the successor of the genesis block
    let block = blocks.next().unwrap().unwrap();
    assert_eq!(
        HEXLOWER.encode(&block.previous_block_hash.0),
        "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
    );
}
