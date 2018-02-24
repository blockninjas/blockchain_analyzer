use super::Repository;
use ::domain::Address;
use ::domain::NewAddress;
use ::schema::addresses;
use ::schema::addresses::dsl::*;
use diesel;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;

pub struct AddressRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> AddressRepository<'a> {
    fn new(connection: &'a PgConnection) -> AddressRepository<'a> {
        AddressRepository {
            connection
        }
    }
}

impl<'a> Repository for AddressRepository<'a> {
    type NewEntity = NewAddress;
    type Entity = Address;

    fn save(&self, new_block: &NewAddress) -> Address {
        diesel::insert_into(addresses::table)
            .values(new_block)
            .get_result(self.connection)
            .expect("Error saving new address")
    }
}