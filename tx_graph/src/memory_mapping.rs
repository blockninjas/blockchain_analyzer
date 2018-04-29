use std::io::Result;
use std::fs::{File, OpenOptions};
use memmap::{Mmap, MmapMut, MmapOptions};

pub fn map_file_into_readable_memory(path: &str) -> Result<Mmap> {
  let file = File::open(path)?;
  let mmap = unsafe { MmapOptions::new().map(&file)? };
  Ok(mmap)
}

pub fn map_file_into_writable_memory(path: &str) -> Result<MmapMut> {
  let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(path)?;

  // TODO If the file does not yet exist, set a non-zero lenght, otherwise it
  // cannot be memory-mapped.

  let mmap = unsafe { MmapOptions::new().map_mut(&file)? };
  Ok(mmap)
}
