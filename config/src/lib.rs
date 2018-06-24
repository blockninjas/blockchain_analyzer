extern crate dotenv;

use dotenv::dotenv;
use std::env;

/// Analysis suite configuration
///
/// Encapsulates configuration values that are commonly used to configure the
/// main components of the analysis suite.
pub struct Config {
  pub db_url: String,
  pub blk_file_path: String,
  pub address_cache_size: usize,
  pub bir_file_path: String,
  pub bir_construction_state_file_path: String,
}

impl Config {
  /// Loads the default configuration.
  pub fn load() -> Config {
    dotenv().ok();
    Config {
      db_url: env::var("DATABASE_URL")
        .expect("DATABASE_URL not set in environment"),
      blk_file_path: env::var("BLK_FILE_PATH")
        .expect("BLK_FILE_PATH not set in environment"),
      address_cache_size: env::var("ADDRESS_CACHE_SIZE")
        .expect("ADDRESS_CACHE_SIZE not set in environment")
        .parse()
        .unwrap(),
      bir_file_path: env::var("BIR_FILE_PATH")
        .expect("BIR_FILE_PATH not set in environment"),
      bir_construction_state_file_path: env::var(
        "BIR_CONSTRUCTION_STATE_FILE_PATH",
      ).expect(
        "BIR_CONSTRUCTION_STATE_FILE_PATH not set in environment",
      ),
    }
  }

  /// Loads the configuration for testing.
  pub fn load_test() -> Config {
    dotenv().ok();
    Config {
      db_url: env::var("TEST_DATABASE_URL").unwrap(),
      blk_file_path: env::var("TEST_BLK_FILE_PATH").unwrap(),
      address_cache_size: env::var("TEST_ADDRESS_CACHE_SIZE")
        .unwrap()
        .parse()
        .unwrap(),
      bir_file_path: env::var("TEST_BIR_FILE_PATH").unwrap(),
      bir_construction_state_file_path: env::var(
        "TEST_BIR_CONSTRUCTION_STATE_FILE_PATH",
      ).unwrap(),
    }
  }
}
