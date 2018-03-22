extern crate blk_file_reader;
extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;

use clap::{App, Arg};
use simplelog::{Config, LogLevelFilter, SimpleLogger};
use std::path::Path;
use std::error::Error;
use blk_file_reader::{read_blk_files, read_blocks};

fn main() {
  let matches = App::new("blk_file_reader")
    .version("0.1.0")
    .about("Read bitcoin blk files")
    .arg(
      Arg::with_name("PATH")
        .required(true)
        .index(1)
        .help("Path to the blk files that should be read"),
    )
    .arg(
      Arg::with_name("full")
        .short("f")
        .long("full")
        .help("Print full block information"),
    )
    .arg(
      // TODO Meaningful if PATH is a directory?
      Arg::with_name("skip")
        .short("s")
        .long("skip")
        .help("Number of blocks to skip")
        .takes_value(true),
    )
    .arg(
      // TODO Enable usage if PATH is a directory.
      Arg::with_name("limit")
        .short("l")
        .long("limit")
        .help("Maximum number of blocks to read")
        .takes_value(true),
    )
    .get_matches();

  configure_logger(&matches);
  let path = matches.value_of("PATH").unwrap();

  if Path::new(path).is_dir() {
    print_blk_files(path);
  } else {
    let number_of_blocks_to_skip = matches
      .value_of("skip")
      .unwrap_or("0")
      .parse::<usize>()
      .unwrap();
    let limit = if matches.is_present("limit") {
      matches.value_of("limit").unwrap().parse::<usize>().unwrap()
    } else {
      usize::max_value()
    };
    print_blk_file(path, number_of_blocks_to_skip, limit);
  }
}

fn configure_logger(matches: &clap::ArgMatches) {
  let log_level = if matches.is_present("full") {
    LogLevelFilter::Debug
  } else {
    LogLevelFilter::Info
  };
  SimpleLogger::init(log_level, Config::default()).unwrap();
}

fn print_blk_files(blk_file_dir: &str) {
  info!("Read blk files at {}", blk_file_dir);
  let mut blk_file_counter = 0;
  // TODO Return error instead of panicking.
  for blk_file in read_blk_files(blk_file_dir).unwrap() {
    print_blk_file(&blk_file, 0, usize::max_value());
    blk_file_counter += 1;
  }
  info!("Processed {} blk files", blk_file_counter);
}

fn print_blk_file(
  blk_file_path: &str,
  number_of_blocks_to_skip: usize,
  limit: usize,
) {
  info!("Read {}", blk_file_path);
  // TODO Return error instead of panicking.
  let blocks = read_blocks(blk_file_path).unwrap();
  let blocks = blocks.skip(number_of_blocks_to_skip);
  let mut block_counter = 0;
  for block in blocks {
    match block {
      Ok(ref block) => debug!("{:#?}", block),
      Err(ref error) => {
        error!("Could not read file (reason: {})", error.description());
        break;
      }
    }
    block_counter += 1;

    if block_counter >= limit {
      break;
    }
  }
  info!("Processed {} blocks in {}", block_counter, blk_file_path);
}
