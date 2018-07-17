use super::Block;
use bincode;
use std::fs::File;
use std::io::BufReader;

pub struct BirFileIterator {
  pub bir_file: BufReader<File>,
}

impl BirFileIterator {
  pub fn new(bir_file: BufReader<File>) -> BirFileIterator {
    BirFileIterator { bir_file }
  }
}

impl Iterator for BirFileIterator {
  type Item = Block;

  fn next(&mut self) -> Option<Block> {
    if let Ok(block) = bincode::deserialize_from(&mut self.bir_file) {
      Some(block)
    } else {
      None
    }
  }
}
