use diesel::{sql_query, QueryDsl, RunQueryDsl, dsl::max, pg::PgConnection};
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
  pub fn max_id(&self) -> Option<i64> {
    // TODO Return error instead of panicking.
    addresses
      .select(max(id))
      .first(self.connection)
      .unwrap()
  }

  pub fn deduplicate_output_addresses(
    &self,
    latest_deduplicated_output_address_id: i64,
  ) {
    let query = format!(
      r"
        insert into addresses (base58check)
          select base58check from output_addresses
            where output_addresses.output_id > {}
            group by base58check
      ",
      latest_deduplicated_output_address_id
    );

    // TODO Return error instead of panicking.
    sql_query(query)
      .execute(self.connection)
      .unwrap();
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
      assert_eq!(Some(latest_address.id), max_id);
      assert_ne!(Some(earlier_address.id), max_id);
      Ok(())
    });
  }
}
