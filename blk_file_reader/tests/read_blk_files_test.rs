//! Tests for the `read_blk_file` function.

extern crate blk_file_reader;

use blk_file_reader::read_blk_files;

#[test]
fn can_read_all_blk_files() {
  // Given
  let path = "sample_blk_files";
  // When
  let blk_files = read_blk_files(path).unwrap();
  // Then
  assert_eq!(blk_files.len(), 4);
  assert_eq!(blk_files[0], "sample_blk_files/blk00000.dat");
  assert_eq!(blk_files[1], "sample_blk_files/blk00001.dat");
  assert_eq!(blk_files[2], "sample_blk_files/blk00931.dat");
  assert_eq!(blk_files[3], "sample_blk_files/blk01208.dat");
}
