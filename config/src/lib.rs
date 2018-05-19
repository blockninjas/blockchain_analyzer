extern crate dotenv;

use dotenv::dotenv;
use std::env;

/// Analysis suite configuration
///
/// Encapsulates configuration values that are commonly used to configure the
/// main components of the analysis suite.
pub struct Config {
  pub db_url: String,
  pub redis_url: String,
  pub blk_file_path: String,
  pub address_cache_size: usize,
}

impl Config {
  /// Loads the default configuration.
  pub fn load() -> Config {
    dotenv().ok();
    Config {
      db_url: env::var("DB_URL").unwrap(),
      redis_url: env::var("REDIS_URL").unwrap(),
      blk_file_path: env::var("BLK_FILE_PATH").unwrap(),
      address_cache_size: env::var("ADDRESS_CACHE_SIZE")
        .unwrap()
        .parse()
        .unwrap(),
    }
  }

  /// Loads the configuration for testing.
  pub fn load_test() -> Config {
    dotenv().ok();
    Config {
      db_url: env::var("TEST_DB_URL").unwrap(),
      redis_url: env::var("TEST_REDIS_URL").unwrap(),
      blk_file_path: env::var("TEST_BLK_FILE_PATH").unwrap(),
      address_cache_size: env::var("TEST_ADDRESS_CACHE_SIZE")
        .unwrap()
        .parse()
        .unwrap(),
    }
  }
}
