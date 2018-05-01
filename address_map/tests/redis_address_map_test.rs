extern crate address_map;
extern crate dotenv;
extern crate redis;

use address_map::{AddressMap, RedisAddressMap};
use redis::Connection;
use std::env;
use dotenv::dotenv;

fn redis_cleanup_db(connection: &Connection, db_id: u32) {
  let _: () = redis::cmd("SELECT").arg(db_id).query(connection).unwrap();
  let _: () = redis::cmd("FLUSHDB").query(connection).unwrap();
}

fn redis_fixture<F>(db_id: u32, mut test_body: F)
where
  F: FnMut(Connection),
{
  dotenv().ok();
  let redis_url = env::var("TEST_REDIS_URL").unwrap();
  let client = redis::Client::open(redis_url.as_str()).unwrap();
  let connection = client.get_connection().unwrap();
  redis_cleanup_db(&connection, db_id);

  test_body(connection);

  let connection = client.get_connection().unwrap();
  redis_cleanup_db(&connection, db_id);
}

#[test]
pub fn can_add_address() {
  redis_fixture(1, |connection| {
    let mut address_map = RedisAddressMap::new(connection);

    let address = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
    let address_id = address_map.get_id(&address);

    assert!(address_id > 0);
  });
}

#[test]
pub fn can_find_existing_address() {
  redis_fixture(2, |connection| {
    let mut address_map = RedisAddressMap::new(connection);

    let address = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
    let address_id1 = address_map.get_id(&address);
    let address_id2 = address_map.get_id(&address);

    assert_eq!(address_id1, address_id2);
  });
}

#[test]
pub fn assigns_different_ids() {
  redis_fixture(3, |connection| {
    let mut address_map = RedisAddressMap::new(connection);

    let address1 = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
    let address_id1 = address_map.get_id(&address1);

    let address2 = "1MavrodizxWNx9gguZzDUKBeHehfhy1goX";
    let address_id2 = address_map.get_id(&address2);
    assert_ne!(address_id1, address_id2);
  });
}
