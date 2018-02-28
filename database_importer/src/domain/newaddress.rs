use ::schema::addresses;
use blk_file_reader;

#[derive(Insertable)]
#[table_name="addresses"]
pub struct NewAddress {
    pub hash: Vec<u8>,
    pub base58_string: String,
    pub output_id: i32,
}

impl NewAddress {
    pub fn new(address: &blk_file_reader::Address, output_id: i32) -> NewAddress {
        NewAddress {
            hash: address.hash.to_vec(),
            base58_string: address.base58_string.clone(),
            output_id,
        }
    }
}
