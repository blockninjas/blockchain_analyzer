use ::schema::blocks;
use blk_file_reader;

#[derive(Insertable)]
#[table_name="blocks"]
pub struct NewBlock {
    pub hash: Vec<u8>,
    pub version: i32,
    pub previous_block_hash: Vec<u8>,
    pub merkle_root: Vec<u8>,
    pub creation_time: i32,
    pub nonce: i32,
}

impl NewBlock {
    pub fn new(block: &blk_file_reader::Block) -> NewBlock {
        NewBlock {
            hash: block.hash.0.to_vec(),
            version: block.version as i32,
            previous_block_hash: block.previous_block_hash.0.to_vec(),
            merkle_root: block.merkle_root.0.to_vec(),
            creation_time: block.creation_time as i32,
            nonce: block.nonce as i32,
        }
    }
}
