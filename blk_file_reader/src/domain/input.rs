use super::Hash;

#[derive(Debug)]
pub struct Input {
    pub sequence_number: u32,
    pub previous_tx_hash: Hash,
    pub previous_tx_output_index: u32,
}
