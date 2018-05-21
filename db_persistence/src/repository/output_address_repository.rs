use diesel::{self, RunQueryDsl, pg::PgConnection};
use domain::{NewOutputAddress, OutputAddress};
use schema::output_addresses;

pub struct OutputAddressRepository<'a> {
  connection: &'a PgConnection,
}

impl<'a> OutputAddressRepository<'a> {
  pub fn new(connection: &'a PgConnection) -> OutputAddressRepository<'a> {
    OutputAddressRepository { connection }
  }

  pub fn save(&self, new_output_address: &NewOutputAddress) -> OutputAddress {
    diesel::insert_into(output_addresses::table)
      .values(new_output_address)
      .get_result(self.connection)
      .expect("Error saving new address")
  }
}
