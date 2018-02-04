#[derive(Debug)]
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

#[derive(Debug)]
pub struct Transaction {
    pub tx_hash: String,
    pub version: u32,
    pub lock_time: u32,
    pub creation_time: u32,
    pub inputs: Box<[Input]>,
    pub outputs: Box<[Output]>,
    pub block_height: u64,
}

#[derive(Debug)]
pub struct Input {
    pub sequence_number: u32,
    pub script: Box<[u8]>,
    pub previous_tx_hash: String,
    pub previous_tx_output_index: u32,
}

#[derive(Debug)]
pub struct Output {
    pub index: u32,
    pub script: Box<[u8]>,
    pub value: u64,
}
