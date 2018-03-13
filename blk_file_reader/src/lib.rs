extern crate clap;
extern crate crypto;
extern crate keys;
#[macro_use]
extern crate log;
extern crate script;

mod domain;
mod block_read;
mod block_reader;

pub use domain::*;
pub use block_read::BlockRead;
pub use block_reader::BlockReader;
use std::path::Path;
use std::error::Error;

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

pub fn read_blk_files(source_path: &str) -> usize {
  let mut blk_file_counter = 0;
  // TODO Return error instead of panicking.
  let blk_files = list_blk_files(source_path).unwrap();
  for blk_file in blk_files.iter() {
    info!("Read {}", blk_file);
    let number_of_blocks = read_blk_file(blk_file);
    info!("Processed {} blocks in {}", number_of_blocks, blk_file);
    blk_file_counter += 1;
  }
  blk_file_counter
}

pub fn read_blk_file(blk_file_path: &str) -> usize {
  let mut block_reader = BlockReader::from_blk_file(blk_file_path);
  let mut block_counter = 0;
  loop {
    if let Err(ref error) = block_reader.read() {
      if error.kind() != std::io::ErrorKind::UnexpectedEof {
        error!("Could not read file (reason: {})", error.description());
      }
      break;
    };
    block_counter += 1;
  }
  block_counter
}
