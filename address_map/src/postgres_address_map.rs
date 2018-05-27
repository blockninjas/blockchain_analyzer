use super::{AddressMap, address_map::Address, address_map::AddressId};
use db_persistence::schema::addresses::dsl::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

pub struct PostgresAddressMap<'a> {
  connection: &'a PgConnection,
}

impl<'a> PostgresAddressMap<'a> {
  pub fn new(connection: &'a PgConnection) -> PostgresAddressMap<'a> {
    PostgresAddressMap { connection }
  }
}

// TODO Breaks semantics as it does not insert new ids.
impl<'a> AddressMap for PostgresAddressMap<'a> {
  fn get_id(&mut self, address: Address) -> AddressId {
    let address_id: i64 = addresses
      .select(id)
      .filter(base58check.eq(address))
      .first(self.connection)
      .unwrap();
    address_id as u64
  }
}
