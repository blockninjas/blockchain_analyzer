extern crate clap;
extern crate db_importer;
#[macro_use]
extern crate log;
extern crate config;
extern crate simplelog;

use clap::{App, Arg};
use db_importer::DbImporter;
use simplelog::{Config, LogLevelFilter, SimpleLogger};

fn main() {
  // TODO Add argument to configure number of threads used by rayon.
  let matches = App::new("db_importer")
    .version("0.1.0")
    .about("Import raw blockchain data into a database.")
    .arg(
      Arg::with_name("debug")
        .short("d")
        .long("debug")
        .help("Print debug information"),
    )
    .get_matches();

  configure_logger(&matches);

  let config = config::Config::load();

  info!(
    "Start importing blk files from {}",
    config.blk_file_path
  );

  let db_importer = DbImporter::new(config);
  db_importer.run();

  info!("Finished import.");
}

fn configure_logger(matches: &clap::ArgMatches) {
  let log_level = if matches.is_present("debug") {
    LogLevelFilter::Debug
  } else {
    LogLevelFilter::Info
  };
  SimpleLogger::init(log_level, Config::default()).unwrap();
}
