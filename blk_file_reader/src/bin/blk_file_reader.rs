extern crate blk_file_reader;
extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;

use clap::{App, Arg};
use simplelog::{Config, LogLevelFilter, SimpleLogger};
use std::path::Path;
use std::error::Error;
use blk_file_reader::{list_blk_files, BlockRead, BlockReader};

fn main() {
  // TODO Introduce `skip` and `limit` flags.
  let matches = App::new("blk_file_reader")
    .version("0.1.0")
    .about("Read bitcoin .blk files")
    .arg(
      Arg::with_name("PATH")
        .required(true)
        .index(1)
        .help("Path to the .blk files that should be read"),
    )
    .arg(
      Arg::with_name("debug")
        .short("d")
        .long("debug")
        .help("Print debug information"),
    )
    .get_matches();

  configure_logger(&matches);
  let path = matches.value_of("PATH").unwrap();

  if Path::new(path).is_dir() {
    read_blk_files(path);
  } else {
    read_blk_file(path);
  }
}

fn configure_logger(matches: &clap::ArgMatches) {
  let log_level = if matches.is_present("debug") {
    LogLevelFilter::Debug
  } else {
    LogLevelFilter::Info
  };
  SimpleLogger::init(log_level, Config::default()).unwrap();
}

fn read_blk_files(blk_file_dir: &str) {
  info!("Start reading .blk files at {}", blk_file_dir);
  let mut blk_file_counter = 0;
  // TODO Return error instead of panicking.
  let blk_files = list_blk_files(blk_file_dir).unwrap();
  for blk_file in blk_files.iter() {
    read_blk_file(blk_file);
    blk_file_counter += 1;
  }
  info!("Processed {} blk files", blk_file_counter);
}

fn read_blk_file(blk_file_path: &str) {
  info!("Read {}", blk_file_path);
  let mut block_reader = BlockReader::from_blk_file(blk_file_path);
  let mut block_counter = 0;
  loop {
    if let Err(ref error) = block_reader.read() {
      if error.kind() != std::io::ErrorKind::UnexpectedEof {
        error!("Could not read file (reason: {})", error.description());
      }
      break;
    };
    block_counter += 1;
  }
  info!("Processed {} blocks in {}", block_counter, blk_file_path);
}
