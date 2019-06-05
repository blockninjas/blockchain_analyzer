use super::Blocks;
use std::io::{self, BufReader};
use std::path::Path;

/// Reads all blk files at the given path.
///
/// Returns an ordered vector of absolute pathes.
///
/// TODO Use `Path` or `OsString` instead of `String`.
pub fn read_blk_files(path_str: &str) -> io::Result<Vec<String>> {
    let mut blk_files = Vec::new();
    let path = Path::new(path_str);
    for dir_entry in path.read_dir().unwrap() {
        let file_name = dir_entry.unwrap().file_name().into_string().unwrap();
        if is_blk_file(&file_name) {
            // TODO Retrieve path via `DirEntry::path()`
            let blk_file_path = format!("{}/{}", path_str, file_name);
            blk_files.push(blk_file_path);
        }
    }
    blk_files.sort();
    Ok(blk_files)
}

fn is_blk_file(file_name: &str) -> bool {
    return file_name.starts_with("blk") && file_name.ends_with(".dat");
}

/// Reads the blocks of the blk file at the given path.
pub fn read_blocks(path_to_blk_file: &str) -> io::Result<Blocks> {
    let file = std::fs::File::open(path_to_blk_file)?;
    let buf_reader = BufReader::new(file);
    Ok(Blocks::new(buf_reader))
}
