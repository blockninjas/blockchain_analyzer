extern crate database_importer;
extern crate clap;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate dotenv;

use clap::{App, Arg};
use simplelog::{SimpleLogger, LogLevelFilter, Config};
use std::error::Error;
use database_importer::import_blk_files;
use dotenv::dotenv;
use std::env;

const DATABASE_URL_ARGUMENT_NAME: &'static str = "database-url";
const DATABASE_URL_ENVIRONMENT_VARIALE_NAME: &'static str = "DATABASE_URL";

fn main() {
    // TODO Add argument to configure number of threads used by rayon.
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
        .arg(Arg::with_name(DATABASE_URL_ARGUMENT_NAME)
             .long(DATABASE_URL_ARGUMENT_NAME)
             .takes_value(true)
             .help(&format!("Specifies the database URL to connect to. Falls back \
                   to the {} environment variable if unspecified.",
                   DATABASE_URL_ENVIRONMENT_VARIALE_NAME)))
        .get_matches();

    configure_logger(&matches);

    let path_str = matches.value_of("PATH").unwrap();
    let database_url = get_database_url(&matches);

    info!("Start importing .blk files from {}", path_str);

    match import_blk_files(path_str, &database_url) {
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

fn get_database_url(matches: &clap::ArgMatches) -> String {
    if matches.is_present(DATABASE_URL_ARGUMENT_NAME) {
        get_database_url_from_matches(matches)
    } else {
        get_database_url_from_environment()
    }
}

fn get_database_url_from_matches(matches: &clap::ArgMatches) -> String {
    let database_url = matches.value_of(DATABASE_URL_ARGUMENT_NAME)
        .unwrap();
    String::from(database_url)
}

fn get_database_url_from_environment() -> String {
    dotenv().ok();
    // TODO Return error instead of panicking.
    env::var(DATABASE_URL_ENVIRONMENT_VARIALE_NAME)
        .expect(&format!("{} not set", DATABASE_URL_ENVIRONMENT_VARIALE_NAME))
}
