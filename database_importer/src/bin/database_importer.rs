extern crate database_importer;
extern crate clap;
#[macro_use] extern crate log;
extern crate simplelog;

use clap::{App, Arg};
use simplelog::{SimpleLogger, LogLevelFilter, Config};
use std::path::Path;

use database_importer::import_blk_files;
use database_importer::import_blk_file;

fn main() {
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
    import_from_path(path_str);
}

fn configure_logger(matches: &clap::ArgMatches) {
    let log_level = if matches.is_present("debug") {
        LogLevelFilter::Debug
    } else {
        LogLevelFilter::Info
    };
    SimpleLogger::init(log_level, Config::default()).unwrap();
}

fn import_from_path(path_str: &str) {
    let path = Path::new(path_str);

    info!("Start importing .blk files from {}", path_str);

    if path.is_dir() {
        let number_of_processed_files = import_blk_files(path_str);
        info!("Processed {} blk files", number_of_processed_files);
    } else if path.is_file() {
        let number_of_processed_blocks = import_blk_file(path_str);
        info!("Processed {} blocks", number_of_processed_blocks);
    } else {
        panic!("{} isn't a valid path!", path_str);
    }
}
