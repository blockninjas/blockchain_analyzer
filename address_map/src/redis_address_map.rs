use address_map::{AddressHash, AddressId, AddressMap};
use redis::{self, Commands, ConnectionLike};

const ADDRESS_ID_COUNTER_KEY: &'static str = "address_id_counter";

/// Redis-based implementation of `AddressMap`.
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
  fn get_address_id(&mut self, address_hash: AddressHash) -> AddressId {
    // First check, without a transaction, if the address hash already exists in
    // the map.
    let address_id: Option<AddressId> =
      self.connection.get(address_hash).unwrap();

    if let Some(address_id) = address_id {
      println!("address {} exists {}", address_hash, address_id);
      // If the address already exists in the map, return its id.
      address_id
    } else {
      println!("address {} does not exist", address_hash);
      // If the address does not yet exist in the map, it has to be inserted,
      // which is done inside a transaction.
      redis::transaction(&self.connection, &[address_hash], |_pipe| {
        // Before actually inserting it, make sure again, that it has not yet
        // been inserted in the meantime.
        let address_id: Option<AddressId> = self.connection.get(address_hash)?;

        if let Some(_) = address_id {
          // If the address already exists in the map, return its id.
          Ok(address_id)
        } else {
          // Otherwise, get the next free id, assign it to the address and
          // return it.
          let next_free_address_id: AddressId =
            self.connection.incr(ADDRESS_ID_COUNTER_KEY, 1)?;
          println!("incremented counter to: {}", next_free_address_id);
          let reply: String =
            self.connection.set(address_hash, next_free_address_id)?;
          println!("reply: {}", reply);
          let address_id2: Option<AddressId> =
            self.connection.get(address_hash).unwrap();
          println!("inserted new id: {}", address_id2.unwrap());
          Ok(Some(next_free_address_id))
        }
      }).unwrap()
    }
  }
}
