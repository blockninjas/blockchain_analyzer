use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub fn read_bir_files<P>(path: P) -> ::std::io::Result<Vec<PathBuf>>
where
  P: AsRef<Path>,
{
  let mut bir_files: Vec<PathBuf> = path
    .as_ref()
    .read_dir()?
    .map(|entry| entry.unwrap().path())
    .filter(|path_buf| {
      if let Some(file_name) = path_buf.file_name() {
        is_bir_file(file_name)
      } else {
        false
      }
    })
    .collect();
  bir_files.sort_unstable();
  Ok(bir_files)
}

fn is_bir_file(file_name: &OsStr) -> bool {
  if let Some(file_name) = file_name.to_str() {
    file_name.len() == 12
      && file_name.starts_with("bir")
      && file_name.ends_with(".dat")
  } else {
    false
  }
}
