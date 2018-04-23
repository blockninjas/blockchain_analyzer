use schema::addresses;
use blk_file_reader;

#[derive(Insertable)]
#[table_name = "addresses"]
pub struct NewAddress {
  pub hash: Vec<u8>,
  pub base58check: String,
  pub output_id: i64,
}

impl NewAddress {
  pub fn new(address: &blk_file_reader::Address, output_id: i64) -> NewAddress {
    NewAddress {
      hash: address.hash.to_vec(),
      base58check: address.base58check.clone(),
      output_id,
    }
  }
}
