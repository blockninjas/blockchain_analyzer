use std::io::{self, BufReader};
use std::fs::File;
use domain::Block;
use primitives::read_block;

/// Allows for iterating over the blocks within a blk file.
pub struct Blocks {
  reader: io::BufReader<File>,
}

impl Blocks {
  pub fn new(reader: BufReader<File>) -> Blocks {
    Blocks { reader }
  }
}

impl Iterator for Blocks {
  type Item = io::Result<Block>;

  fn next(&mut self) -> Option<Self::Item> {
    match read_block(&mut self.reader) {
      Ok(block) => Some(Ok(block)),
      Err(error) => {
        if error.kind() == io::ErrorKind::UnexpectedEof {
          None
        } else {
          Some(Err(error))
        }
      }
    }
  }
}
