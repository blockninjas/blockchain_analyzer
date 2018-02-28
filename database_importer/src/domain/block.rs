#[derive(Queryable)]
pub struct Block {
    pub id: i32,
    pub hash: Vec<u8>,
    pub version: i32,
    pub previous_block_hash: Vec<u8>,
    pub merkle_root: Vec<u8>,
    pub creation_time: i32,
    pub nonce: i32,
    pub height: Option<i32>,
}
