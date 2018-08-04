use dotenv::dotenv;
use failure::Error;
use std::env;

/// Analysis suite configuration
///
/// Encapsulates configuration values that are commonly used to configure the
/// main components of the analysis suite.
#[derive(Debug)]
pub struct Config {
  pub db_url: String,
  pub max_db_connections: u32,
  pub blk_file_path: String,
  pub address_cache_size: usize,
  pub unresolved_bir_file_path: String,
  pub resolved_bir_file_path: String,
  pub bir_construction_state_file_path: String,
}

impl Config {
  /// Loads the default configuration.
  pub fn load() -> Result<Config, Error> {
    dotenv().ok();
    let config = Config {
      db_url: env::var("DATABASE_URL")?,
      max_db_connections: env::var("MAX_DB_CONNECTIONS")?.parse()?,
      blk_file_path: env::var("BLK_FILE_PATH")?,
      address_cache_size: env::var("ADDRESS_CACHE_SIZE")?.parse()?,
      unresolved_bir_file_path: env::var("UNRESOLVED_BIR_FILE_PATH")?,
      resolved_bir_file_path: env::var("RESOLVED_BIR_FILE_PATH")?,
      bir_construction_state_file_path: env::var(
        "BIR_CONSTRUCTION_STATE_FILE_PATH",
      )?,
    };

    Ok(config)
  }

  /// Loads the configuration for testing.
  pub fn load_test() -> Result<Config, Error> {
    dotenv().ok();
    let config = Config {
      db_url: env::var("TEST_DATABASE_URL")?,
      max_db_connections: env::var("MAX_DB_CONNECTIONS")?.parse()?,
      blk_file_path: env::var("TEST_BLK_FILE_PATH")?,
      address_cache_size: env::var("TEST_ADDRESS_CACHE_SIZE")?.parse()?,
      unresolved_bir_file_path: env::var("UNRESOLVED_BIR_FILE_PATH")?,
      resolved_bir_file_path: env::var("RESOLVED_BIR_FILE_PATH")?,
      bir_construction_state_file_path: env::var(
        "TEST_BIR_CONSTRUCTION_STATE_FILE_PATH",
      )?,
    };

    Ok(config)
  }
}
