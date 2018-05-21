use blk_file_reader;
use schema::output_addresses;

#[derive(Insertable)]
#[table_name = "output_addresses"]
pub struct NewOutputAddress {
  pub output_id: i64,
  pub hash: Vec<u8>,
  pub base58check: String,
}

impl NewOutputAddress {
  pub fn new(
    address: &blk_file_reader::Address,
    output_id: i64,
  ) -> NewOutputAddress {
    NewOutputAddress {
      output_id,
      hash: address.hash.to_vec(),
      base58check: address.base58check.clone(),
    }
  }
}
