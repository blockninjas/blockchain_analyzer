use super::{address_map::Address, address_map::AddressId, AddressMap};
use db_persistence::schema;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use std::collections::HashMap;

const ADDRESS_CHUNK_SIZE: usize = 5000;

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
        let address_id: i64 = schema::addresses::dsl::addresses
            .select(schema::addresses::dsl::id)
            .filter(schema::addresses::dsl::base58check.eq(address))
            .first(self.connection)
            .unwrap();
        address_id as u64
    }

    fn get_ids(&mut self, base58check_addresses: &[String]) -> HashMap<String, AddressId> {
        base58check_addresses
            .chunks(ADDRESS_CHUNK_SIZE)
            .flat_map(|chunk| load_ids_for_addresses(self.connection, chunk))
            .map(|(base58check, address_id)| (base58check, address_id as u64))
            .collect()
    }
}

fn load_ids_for_addresses(
    db_connection: &PgConnection,
    base58check_addresses: &[String],
) -> Vec<(String, i64)> {
    schema::addresses::dsl::addresses
        .select((
            schema::addresses::dsl::base58check,
            schema::addresses::dsl::id,
        ))
        .filter(schema::addresses::dsl::base58check.eq_any(base58check_addresses))
        .get_results(db_connection)
        .unwrap()
}
