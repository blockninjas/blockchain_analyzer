use std::io::Result;
use std::fs::{File, OpenOptions};
use memmap::{Mmap, MmapMut, MmapOptions};
use std::path::Path;

const SIZE_OF_NEW_MEMORY_MAPPED_FILE: u64 = 1;

pub fn map_file_into_readable_memory<P>(path: P) -> Result<Mmap>
where
  P: AsRef<Path>,
{
  let file = File::open(path)?;
  let mmap = unsafe { MmapOptions::new().map(&file) };
  mmap
}

pub fn map_file_into_writable_memory<P>(path: P) -> Result<MmapMut>
where
  P: AsRef<Path>,
{
  let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(path)?;

  // Set a non-zero length for the newly created file, otherwise it cannot be
  // memory-mapped.
  file.set_len(SIZE_OF_NEW_MEMORY_MAPPED_FILE)?;

  let mmap = unsafe { MmapOptions::new().map_mut(&file) };
  mmap
}

#[cfg(test)]
mod test {

  extern crate tempdir;

  use super::*;
  use self::tempdir::TempDir;

  #[test]
  fn can_map_new_file_into_writable_memory() {
    // Given
    let dir = TempDir::new("memory_map_test").unwrap();
    let file_path = dir.path().join("new_file");
    println!("{:#?}", file_path);

    // When
    let mmap = map_file_into_writable_memory(file_path);

    // Then
    assert!(mmap.is_err());
  }
}
