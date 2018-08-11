use diesel::{self, dsl::max, pg::PgConnection, QueryDsl, RunQueryDsl};
use domain::{NewOutputAddress, OutputAddress};
use schema::output_addresses;
use schema::output_addresses::dsl::*;
use std::result::Result;

pub struct OutputAddressRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> OutputAddressRepository<'a> {
    pub fn new(connection: &'a PgConnection) -> OutputAddressRepository<'a> {
        OutputAddressRepository { connection }
    }

    pub fn save(
        &self,
        new_output_address: &NewOutputAddress,
    ) -> Result<OutputAddress, diesel::result::Error> {
        diesel::insert_into(output_addresses::table)
            .values(new_output_address)
            .get_result(self.connection)
    }

    /// Returns the maximal output address id, or `None` if no address exists yet.
    pub fn max_id(&self) -> Result<Option<i64>, diesel::result::Error> {
        // TODO Return error instead of panicking.
        output_addresses
            .select(max(output_id))
            .first(self.connection)
    }
}
