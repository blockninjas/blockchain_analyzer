use super::{address_map::Address, address_map::AddressId, AddressMap};
use db_persistence::schema;
use diesel::{self, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use r2d2;
use r2d2_diesel;
use std::collections::HashMap;

const ADDRESS_CHUNK_SIZE: usize = 5000;

impl AddressMap for PgConnection {
    fn get_id(&self, address: Address) -> AddressId {
        let address_id: i64 = schema::addresses::dsl::addresses
            .select(schema::addresses::dsl::id)
            .filter(schema::addresses::dsl::base58check.eq(address))
            .first(self)
            .unwrap();
        address_id as u64
    }

    fn get_ids(&self, base58check_addresses: &[String]) -> HashMap<String, AddressId> {
        base58check_addresses
            .chunks(ADDRESS_CHUNK_SIZE)
            .flat_map(|chunk| load_ids_for_addresses(self, chunk))
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
        )).filter(schema::addresses::dsl::base58check.eq_any(base58check_addresses))
        .get_results(db_connection)
        .unwrap()
}

impl AddressMap for r2d2::PooledConnection<r2d2_diesel::ConnectionManager<diesel::PgConnection>> {
    fn get_id(&self, address: Address) -> AddressId {
        <PgConnection as AddressMap>::get_id(self, address)
    }

    fn get_ids(&self, base58check_addresses: &[String]) -> HashMap<String, AddressId> {
        <PgConnection as AddressMap>::get_ids(self, base58check_addresses)
    }
}
