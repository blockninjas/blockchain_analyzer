use std::io::Result;
use std::fs::File;
use memmap::{Mmap, MmapOptions};
use std::path::Path;

pub fn map_file_into_readable_memory<P>(path: P) -> Result<Mmap>
where
  P: AsRef<Path>,
{
  let file = File::open(path)?;
  let mmap = unsafe { MmapOptions::new().map(&file) };
  mmap
}
