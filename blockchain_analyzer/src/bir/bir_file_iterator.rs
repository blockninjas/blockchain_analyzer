use super::Block;
use bincode;
use std::io::Read;

pub struct BirFileIterator<R>
where
    R: Read,
{
    pub bir_file: R,
}

impl<R> BirFileIterator<R>
where
    R: Read,
{
    pub fn new(bir_file: R) -> BirFileIterator<R> {
        BirFileIterator { bir_file }
    }
}

impl<R> Iterator for BirFileIterator<R>
where
    R: Read,
{
    type Item = Block;

    fn next(&mut self) -> Option<Block> {
        if let Ok(block) = bincode::deserialize_from(&mut self.bir_file) {
            Some(block)
        } else {
            None
        }
    }
}
