use super::{AddressMap, address_map::Address, address_map::AddressId,
            redis_id_map::RedisIdMap};
use redis::{Commands, ConnectionLike};

const ADDRESS_ID_COUNTER_KEY: &'static str = "address_id_counter";

pub struct RedisAddressMap<C>
where
  C: ConnectionLike + Commands,
{
  redis_id_map: RedisIdMap<C>,
}

impl<C> RedisAddressMap<C>
where
  C: ConnectionLike + Commands,
{
  pub fn new(connection: C) -> RedisAddressMap<C> {
    RedisAddressMap {
      redis_id_map: RedisIdMap::new(
        connection,
        String::from(ADDRESS_ID_COUNTER_KEY),
      ),
    }
  }
}

impl<C> AddressMap for RedisAddressMap<C>
where
  C: ConnectionLike + Commands,
{
  fn get_id(&mut self, address: Address) -> AddressId {
    self.redis_id_map.get_id(address)
  }
}
