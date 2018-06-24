use super::{address_map::Address, address_map::AddressId, AddressMap};
use db_persistence::schema::addresses::dsl::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

pub struct PostgresAddressMap {
  connection: PgConnection,
}

impl PostgresAddressMap {
  pub fn new(connection: PgConnection) -> PostgresAddressMap {
    PostgresAddressMap { connection }
  }
}

// TODO Breaks semantics as it does not insert new ids.
impl AddressMap for PostgresAddressMap {
  fn get_id(&mut self, address: Address) -> AddressId {
    let address_id: i64 = addresses
      .select(id)
      .filter(base58check.eq(address))
      .first(&self.connection)
      .unwrap();
    address_id as u64
  }
}
