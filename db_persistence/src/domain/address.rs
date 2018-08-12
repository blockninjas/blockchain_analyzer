use diesel::{self, dsl::max, pg::PgConnection, QueryDsl, RunQueryDsl};
use schema::addresses::dsl::*;
use std::result::Result;

#[derive(Queryable)]
pub struct Address {
    pub id: i64,
    pub base58check: String,
    pub cluster_representative: Option<i64>,
}

impl Address {
    /// Returns the maximal address id, or `None` if no address exists yet.
    // TODO Use `AddressId` instead of `u64`.
    pub fn max_id(db_connection: &PgConnection) -> Result<Option<i64>, diesel::result::Error> {
        // TODO Return error instead of panicking.
        addresses.select(max(id)).first(db_connection)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use diesel::{self, result::Error, Connection, ExpressionMethods};
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
            let max_id = Address::max_id(&db_connection).unwrap();

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
            let max_id = Address::max_id(&db_connection).unwrap();

            // Then
            assert_eq!(Some(latest_address.id), max_id);
            assert_ne!(Some(earlier_address.id), max_id);
            Ok(())
        });
    }
}
