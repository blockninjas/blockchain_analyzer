extern crate dotenv;
extern crate redis;

use dotenv::dotenv;
use std::env;

pub fn redis_test<F>(mut test_body: F)
where
  F: FnMut(redis::Connection),
{
  dotenv().ok();
  let redis_url = env::var("TEST_REDIS_URL").unwrap();
  let client = redis::Client::open(redis_url.as_str()).unwrap();
  let connection = client.get_connection().unwrap();

  redis_cleanup_db(&connection);

  test_body(connection);

  let connection = client.get_connection().unwrap();
  redis_cleanup_db(&connection);
}

fn redis_cleanup_db(connection: &redis::Connection) {
  let _: () = redis::cmd("FLUSHDB").query(connection).unwrap();
}
