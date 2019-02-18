use super::{address_map::Address, address_map::AddressId, AddressMap};
use db::{self, schema};
use diesel::{self, prelude::*};
use failure::Error;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct InMemoryAddressMap {
    addresses: HashMap<String, AddressId>,
}

impl InMemoryAddressMap {
    pub fn new(addresses: HashMap<String, AddressId>) -> InMemoryAddressMap {
        InMemoryAddressMap { addresses }
    }
}

impl AddressMap for InMemoryAddressMap {
    fn get_id(&self, address: Address) -> AddressId {
        *self.addresses.get(address).unwrap()
    }

    fn get_ids(&self, addresses: &[String]) -> HashMap<String, AddressId> {
        let mut ids = HashMap::<String, AddressId>::new();
        for address in addresses {
            if let Some(id) = self.addresses.get(address) {
                ids.insert(address.clone(), *id);
            }
        }
        ids
    }
}

pub fn load_all_addresses(
    db_connection_pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<HashMap<String, u64>, Error> {
    let max_id = {
        let db_connection = db_connection_pool.get()?;
        if let Some(max_id) = db::Address::max_id(&db_connection)? {
            max_id
        } else {
            0
        }
    };

    let limit = 100_000;

    info!("Begin loading all addresses");

    let offsets: Vec<usize> = (0..(max_id + 1) as usize).collect();
    let offsets: Vec<usize> = offsets
        .chunks(limit)
        .map(|chunk| *chunk.first().unwrap())
        .collect();

    let address_chunks: Vec<Vec<(String, u64)>> = offsets
        .par_iter()
        .map(|offset| {
            // TODO Return error instead of panicking.
            let db_connection = db_connection_pool.get().unwrap();

            let chunk: Vec<(String, u64)> =
                load_addresses_in_range(&db_connection, *offset as i64, limit as i64)
                    .unwrap()
                    .into_iter()
                    .map(|(base58check, address_id)| (base58check, address_id as u64))
                    .collect();
            info!("Loaded {} addresses from offset {}", chunk.len(), offset);
            chunk
        })
        .collect();

    info!("Collect addresses into hash map");

    let mut addresses = HashMap::with_capacity(max_id as usize);

    for mut address_chunk in address_chunks {
        addresses.extend(address_chunk);
    }

    info!("Loaded {} addresses", addresses.len());

    Ok(addresses)
}

fn load_addresses_in_range(
    db_connection: &PgConnection,
    id: i64,
    number_of_addresses: i64,
) -> Result<Vec<(String, i64)>, diesel::result::Error> {
    schema::addresses::dsl::addresses
        .select((
            schema::addresses::dsl::base58check,
            schema::addresses::dsl::id,
        ))
        .filter(
            schema::addresses::dsl::id
                .ge(id)
                .and(schema::addresses::dsl::id.lt(id + number_of_addresses)),
        )
        .get_results(db_connection)
}
