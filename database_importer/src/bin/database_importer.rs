extern crate database_importer;
extern crate clap;
#[macro_use] extern crate log;
extern crate simplelog;

use clap::{App, Arg};
use simplelog::{SimpleLogger, LogLevelFilter, Config};
use std::error::Error;
use database_importer::import_blk_files;

fn main() {
    // TODO Pass DATABASE_URL as argument.
    let matches = App::new("database_importer")
        .version("0.1.0")
        .about("Import raw blockchain data into a database.")
        .arg(Arg::with_name("PATH")
             .required(true)
             .index(1)
             .help("Path to the .blk files that should be read"))
        .arg(Arg::with_name("debug")
             .short("d")
             .long("debug")
             .help("Print debug information"))
        .get_matches();

    configure_logger(&matches);

    let path_str = matches.value_of("PATH").unwrap();

    info!("Start importing .blk files from {}", path_str);

    match import_blk_files(path_str) {
        Ok(_) => {
            info!("Finished import.");
        },
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
