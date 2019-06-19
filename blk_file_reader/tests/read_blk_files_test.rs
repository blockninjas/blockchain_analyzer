//! Tests for the `read_blk_file` function.

extern crate blk_file_reader;

use blk_file_reader::read_blk_files;

#[test]
fn can_read_all_blk_files() {
    // Given
    let path = "../sample-blk-files";
    // When
    let blk_files = read_blk_files(path).unwrap();
    // Then
    assert_eq!(blk_files.len(), 7);
    assert_eq!(blk_files[0], "../sample-blk-files/blk00000.dat");
    assert_eq!(blk_files[1], "../sample-blk-files/blk00001.dat");
    assert_eq!(blk_files[2], "../sample-blk-files/blk00002.dat");
    assert_eq!(blk_files[3], "../sample-blk-files/blk00003.dat");
    assert_eq!(blk_files[4], "../sample-blk-files/blk00004.dat");
    assert_eq!(blk_files[5], "../sample-blk-files/blk00931.dat");
    assert_eq!(blk_files[6], "../sample-blk-files/blk01208.dat");
}
