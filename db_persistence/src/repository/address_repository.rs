use diesel::{QueryDsl, RunQueryDsl, dsl::max, pg::PgConnection};
use schema::addresses::dsl::*;

pub struct AddressRepository<'a> {
  connection: &'a PgConnection,
}

impl<'a> AddressRepository<'a> {
  pub fn new(connection: &'a PgConnection) -> AddressRepository<'a> {
    AddressRepository { connection }
  }

  /// Returns the maximal address id, or `None` if no address exists yet.
  // TODO Use `AddressId` instead of `u64`.
  pub fn max_id(&self) -> Option<u64> {
    // TODO Return error instead of panicking.
    let max_id: Option<i64> = addresses
      .select(max(id))
      .first(self.connection)
      .unwrap();

    if let Some(max_id) = max_id {
      Some(max_id as u64)
    } else {
      None
    }
  }
}

#[cfg(test)]
mod test {

  use super::*;
  use diesel::{self, Connection, ExpressionMethods, result::Error};
  use domain::Address;

  // TODO Make database URL configurable.
  const TEST_DATABASE_URL: &'static str =
    "postgres://postgres:test@127.0.0.1:5432/bitcoin_blockchain";

  #[test]
  pub fn max_id_returns_none_if_no_address_exists() {
    // Given
    let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();

    db_connection.test_transaction::<_, Error, _>(|| {
      // When
      let address_repository = AddressRepository::new(&db_connection);
      let max_id = address_repository.max_id();

      // Then
      assert_eq!(None, max_id);
      Ok(())
    });
  }

  #[test]
  pub fn max_id_returns_id_of_latest_insert() {
    // Given
    let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();

    db_connection.test_transaction::<_, Error, _>(|| {
      let earlier_address: Address = diesel::insert_into(addresses)
        .values(base58check.eq("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"))
        .get_result(&db_connection)
        .unwrap();

      let latest_address: Address = diesel::insert_into(addresses)
        .values(base58check.eq("12c6DSiU4Rq3P4ZxziKxzrL5LmMBrzjrJX"))
        .get_result(&db_connection)
        .unwrap();

      // When
      let address_repository = AddressRepository::new(&db_connection);
      let max_id = address_repository.max_id();

      // Then
      assert_eq!(Some(latest_address.id as u64), max_id);
      assert_ne!(Some(earlier_address.id as u64), max_id);
      Ok(())
    });
  }
}
