pub struct Block {
    pub hash: String,
    pub version: u32,
    pub previous_block_hash: String,
    pub merkle_root: String,
    pub creation_time: u32,
    pub bits: u32,
    pub nonce: u32,
    pub block_height: u64,
    pub transactions: Box<[Transaction]>,
}

pub struct Transaction {
    pub tx_hash: String,
    pub version: u8,
    pub lock_time: u32,
    pub creation_time: u32,
    pub input_count: u32,
    pub output_count: u32,
    pub block_height: u64,
}

pub struct Input {
    pub tx_hash: String,
    pub sequence_number: u32,
    pub address: String,
    pub script: Box<[u8]>,
    pub previous_tx_hash: String,
    pub output_index: u32,
}

pub struct Output {
    pub tx_hash: String,
    pub sequence_number: u32,
    pub address: String,
    pub script: Box<[u8]>,
    pub value: u32,
}
