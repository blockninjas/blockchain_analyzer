use domain::Block;
use read::ReadBlock;
use std::fs::File;
use std::io::{self, BufReader};

/// Allows for iterating over the blocks within a blk file.
pub struct Blocks {
    reader: io::BufReader<File>,
    index_in_blk_file: usize,
}

impl Blocks {
    pub fn new(reader: BufReader<File>) -> Blocks {
        Blocks {
            reader,
            index_in_blk_file: 0,
        }
    }
}

impl Iterator for Blocks {
    type Item = io::Result<Block>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read_block(self.index_in_blk_file) {
            Ok(block) => {
                self.index_in_blk_file += 1;
                Some(Ok(block))
            }
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
