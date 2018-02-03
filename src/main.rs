extern crate blk_file_reader;
extern crate clap;
#[macro_use] extern crate log;
extern crate simplelog;

use clap::{App, Arg};
use simplelog::{SimpleLogger, LogLevelFilter, Config};
use std::path::Path;

use blk_file_reader::read_blk_files;

fn main() {
    let matches = App::new("blk_file_reader")
        .version("0.1.0")
        .about("Read bitcoin .blk files")
        .arg(Arg::with_name("source")
             .short("s")
             .long("source")
             .value_name("PATH")
             .required(true)
             .help("Path to the .blk files that should be read"))
        .arg(Arg::with_name("debug")
             .short("d")
             .long("debug")
             .help("Print debug information"))
        .get_matches();

    configure_logger(&matches);
    let source_path = matches.value_of("source").unwrap();

    if !Path::new(source_path).is_dir() {
        panic!("{} is no directory", source_path);
    }

    info!("Start reading .blk files at {}", source_path);

    let number_of_processed_files = read_blk_files(source_path);

    info!("Processed {} blk files", number_of_processed_files);
}

fn configure_logger(matches: &clap::ArgMatches) {
    let log_level = if matches.is_present("debug") {
        LogLevelFilter::Debug
    } else {
        LogLevelFilter::Info
    };
    SimpleLogger::init(log_level, Config::default()).unwrap();
}

