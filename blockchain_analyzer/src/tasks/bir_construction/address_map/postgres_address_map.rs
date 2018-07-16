use super::{address_map::Address, address_map::AddressId, AddressMap};
use db_persistence::schema::addresses::dsl::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

pub struct PostgresAddressMap<'conn> {
  connection: &'conn PgConnection,
}

impl<'conn> PostgresAddressMap<'conn> {
  pub fn new(connection: &'conn PgConnection) -> PostgresAddressMap<'conn> {
    PostgresAddressMap { connection }
  }
}

impl<'conn> AddressMap for PostgresAddressMap<'conn> {
  fn get_id(&mut self, address: Address) -> AddressId {
    let address_id: i64 = addresses
      .select(id)
      .filter(base58check.eq(address))
      .first(self.connection)
      .unwrap();
    address_id as u64
  }
}
