use diesel::{self, QueryDsl, RunQueryDsl, dsl::max, pg::PgConnection};
use domain::{NewOutputAddress, OutputAddress};
use schema::output_addresses;
use schema::output_addresses::dsl::*;

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

  /// Returns the maximal output address id, or `None` if no address exists yet.
  pub fn max_id(&self) -> Option<i64> {
    // TODO Return error instead of panicking.
    output_addresses
      .select(max(output_id))
      .first(self.connection)
      .unwrap()
  }
}
