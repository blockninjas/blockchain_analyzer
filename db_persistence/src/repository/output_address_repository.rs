use super::Repository;
use diesel;
use diesel::RunQueryDsl;
use diesel::pg::PgConnection;
use domain::NewOutputAddress;
use domain::OutputAddress;
use schema::output_addresses;

pub struct OutputAddressRepository<'a> {
  connection: &'a PgConnection,
}

impl<'a> OutputAddressRepository<'a> {
  pub fn new(connection: &'a PgConnection) -> OutputAddressRepository<'a> {
    OutputAddressRepository { connection }
  }
}

impl<'a> Repository for OutputAddressRepository<'a> {
  type NewEntity = NewOutputAddress;
  type Entity = OutputAddress;

  fn save(&self, new_output_address: &NewOutputAddress) -> OutputAddress {
    diesel::insert_into(output_addresses::table)
      .values(new_output_address)
      .get_result(self.connection)
      .expect("Error saving new address")
  }
}
