extern crate address_map;
extern crate redis;
extern crate redis_test;

use address_map::{AddressMap, RedisAddressMap};
use redis_test::redis_test;

#[test]
pub fn can_add_address() {
  redis_test(|connection| {
    let mut address_map = RedisAddressMap::new(connection);

    let address = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
    let address_id = address_map.get_id(&address);

    assert!(address_id > 0);
  });
}

#[test]
pub fn can_find_existing_address() {
  redis_test(|connection| {
    let mut address_map = RedisAddressMap::new(connection);

    let address = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
    let address_id1 = address_map.get_id(&address);
    let address_id2 = address_map.get_id(&address);

    assert_eq!(address_id1, address_id2);
  });
}

#[test]
pub fn assigns_different_ids() {
  redis_test(|connection| {
    let mut address_map = RedisAddressMap::new(connection);

    let address1 = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
    let address_id1 = address_map.get_id(&address1);

    let address2 = "1MavrodizxWNx9gguZzDUKBeHehfhy1goX";
    let address_id2 = address_map.get_id(&address2);
    assert_ne!(address_id1, address_id2);
  });
}
