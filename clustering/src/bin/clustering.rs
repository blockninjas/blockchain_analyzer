#[macro_use]
extern crate log;
extern crate bir;
extern crate bir_construction;
extern crate blk_file_reader;
extern crate clustering;
extern crate config;
extern crate simplelog;

use config::Config;
use simplelog::{LogLevelFilter, SimpleLogger};

fn main() {
  let config = Config::load();

  SimpleLogger::init(
    LogLevelFilter::Info,
    simplelog::Config::default(),
  ).unwrap();

  info!(
    "start clustering blk files from {}",
    config.blk_file_path
  );
}
