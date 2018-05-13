use super::{AddressMap, address_map::Address, address_map::AddressId};
use redis::{self, Commands, ConnectionLike};

const ADDRESS_ID_COUNTER_KEY: &'static str = "address_id_counter";
// TODO Use prefix for keys.

pub struct RedisAddressMap<C>
where
  C: ConnectionLike + Commands,
{
  connection: C,
}

impl<C> RedisAddressMap<C>
where
  C: ConnectionLike + Commands,
{
  pub fn new(connection: C) -> RedisAddressMap<C> {
    RedisAddressMap { connection }
  }
}

impl<C> AddressMap for RedisAddressMap<C>
where
  C: ConnectionLike + Commands,
{
  fn get_id(&mut self, address: Address) -> AddressId {
    // First check, without a transaction, if the address already exists in the
    // map.
    let address_id: Option<AddressId> = self.connection.get(address).unwrap();

    if let Some(address_id) = address_id {
      // If the address already exists in the map, return its id.
      address_id
    } else {
      // If the address does not yet exist in the map, it has to be inserted,
      // which is done inside a transaction.
      redis::transaction(&self.connection, &[address], |_| {
        // Before actually inserting it, make sure again, that it has not yet
        // been inserted in the meantime.
        let address_id: Option<u64> = self.connection.get(address)?;

        if let Some(_) = address_id {
          // If the address already exists in the map, return its id.
          Ok(address_id)
        } else {
          // Otherwise, get the next free id, assign it to the address and
          // return it.
          let next_free_address_id: u64 =
            self.connection.incr(ADDRESS_ID_COUNTER_KEY, 1)?;
          let _: () = self
            .connection
            .set(address, next_free_address_id)?;
          Ok(Some(next_free_address_id))
        }
      }).unwrap()
    }
  }
}
