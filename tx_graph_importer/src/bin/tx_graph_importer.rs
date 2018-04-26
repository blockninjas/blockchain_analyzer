extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;

use clap::{App, Arg};
use simplelog::{Config, LogLevelFilter, SimpleLogger};

fn main() {
  let matches = App::new("tx_graph_importer")
    .version("0.1.0")
    .about("Import raw blockchain data into the tx_graph.")
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

  let path_str = matches.value_of("PATH").unwrap();

  info!("Start importing blk files from {}", path_str);

  // TODO Implement
}

fn configure_logger(matches: &clap::ArgMatches) {
  let log_level = if matches.is_present("debug") {
    LogLevelFilter::Debug
  } else {
    LogLevelFilter::Info
  };
  SimpleLogger::init(log_level, Config::default()).unwrap();
}
