use std::fmt;

pub struct Hash(
    pub [u8; 32]
);

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

impl Hash {
    pub fn to_hex_string(&self) -> String {
        to_hex_string(&self.0)
    }
}

fn to_hex_string(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect()
}

#[cfg(test)]
mod to_hex_string_tests {
    use super::to_hex_string;

    #[test]
    fn returns_big_endian_hex() {
        // given
        let bytes = [0x89, 0xAB, 0xCD, 0xEF];

        // when
        let actual_hex = to_hex_string(&bytes);

        // then
        let expected_hex = "89ABCDEF";
        assert_eq!(expected_hex, actual_hex);
    }

    #[test]
    fn does_not_truncate_leading_zeros() {
        // given
        let bytes = [0x00, 0x01];

        // when
        let actual_hex = to_hex_string(&bytes);

        // then
        let expected_hex = "0001";
        assert_eq!(expected_hex, actual_hex);
    }
}

#[derive(Debug)]
pub struct Block {
    pub hash: Hash,
    pub version: u32,
    pub previous_block_hash: Hash,
    pub merkle_root: Hash,
    pub creation_time: u32,
    pub bits: u32,
    pub nonce: u32,
    pub block_height: u64,
    pub transactions: Box<[Transaction]>,
}

#[derive(Debug)]
pub struct Transaction {
    pub tx_hash: Hash,
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
    pub previous_tx_hash: Hash,
    pub previous_tx_output_index: u32,
}

#[derive(Debug)]
pub struct Output {
    pub index: u32,
    pub script: Box<[u8]>,
    pub value: u64,
}
