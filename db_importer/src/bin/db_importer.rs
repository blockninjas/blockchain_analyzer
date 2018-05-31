extern crate clap;
extern crate db_importer;
#[macro_use]
extern crate log;
extern crate config;
extern crate simplelog;

use clap::{App, Arg};
use db_importer::import_blk_files;
use simplelog::{Config, LogLevelFilter, SimpleLogger};
use std::error::Error;

fn main() {
  // TODO Add argument to configure number of threads used by rayon.
  let matches = App::new("db_importer")
    .version("0.1.0")
    .about("Import raw blockchain data into a database.")
    .arg(
      Arg::with_name("PATH")
        .required(true)
        .index(1)
        .help("Path to the blk files that should be read"),
    )
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

  match import_blk_files(&config) {
    Ok(_) => {
      info!("Finished import.");
    }
    Err(ref error) => {
      error!("{}", error.description());
      std::process::exit(1);
    }
  };
}

fn configure_logger(matches: &clap::ArgMatches) {
  let log_level = if matches.is_present("debug") {
    LogLevelFilter::Debug
  } else {
    LogLevelFilter::Info
  };
  SimpleLogger::init(log_level, Config::default()).unwrap();
}
