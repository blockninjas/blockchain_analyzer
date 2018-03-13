extern crate crypto;
extern crate keys;
extern crate script;

mod domain;
mod block_read;
mod block_reader;

pub use domain::*;
pub use block_read::BlockRead;
pub use block_reader::BlockReader;
use std::path::Path;

/// List all .blk files within the directory at the given path.
///
/// The returned vector contains the path to each .blk file, relative to
/// `path_str`.
///
/// TODO Use `Path` or `OsString` instead of `String`.
pub fn list_blk_files(path_str: &str) -> std::io::Result<Vec<String>> {
  let mut blk_files = Vec::new();
  let path = Path::new(path_str);
  for dir_entry in path.read_dir().unwrap() {
    let file_name = dir_entry.unwrap().file_name().into_string().unwrap();
    if file_name.starts_with("blk") && file_name.ends_with(".dat") {
      // TODO Retrieve path via `DirEntry::path()`
      let blk_file_path = format!("{}/{}", path_str, file_name);
      blk_files.push(blk_file_path);
    }
  }
  blk_files.sort();
  Ok(blk_files)
}
