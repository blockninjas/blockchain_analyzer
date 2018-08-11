use diesel::{
  self, pg::PgConnection, ExpressionMethods, OptionalExtension, QueryDsl,
  RunQueryDsl,
};
use schema::address_deduplicator_states::dsl::*;
use std::result::Result;

pub struct AddressDeduplicatorStateRepository<'a> {
  connection: &'a PgConnection,
}

impl<'a> AddressDeduplicatorStateRepository<'a> {
  pub fn new(
    connection: &'a PgConnection,
  ) -> AddressDeduplicatorStateRepository<'a> {
    AddressDeduplicatorStateRepository { connection }
  }

  /// Saves a new address deduplicator state with the given `output_address_id`.
  pub fn save(
    &self,
    new_output_address_id: i64,
  ) -> Result<usize, diesel::result::Error> {
    // TODO Return error instead of panicking.
    diesel::insert_into(address_deduplicator_states)
      .values(output_address_id.eq(new_output_address_id))
      .execute(self.connection)
  }

  /// Returns the id of the latest deduplicated output address or `None` if none
  /// have been deduplicated so far.
  pub fn latest(&self) -> Result<Option<i64>, diesel::result::Error> {
    // TODO Return error instead of panicking.
    address_deduplicator_states
      .select(output_address_id)
      .order(id.desc())
      .first(self.connection)
      .optional()
  }
}
