extern crate blockchain_analyzer;
extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;

use blockchain_analyzer::tasks::{
  AddressDeduplicationTask, BirConstructionTask, BlkFileImportTask,
  BlockHeightCalculationTask, ClusteringTask,
};
use blockchain_analyzer::{task_manager, Config};
use clap::{App, Arg};
use simplelog::{LogLevelFilter, SimpleLogger};

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

  let config = Config::load();

  info!("Start importing blk files from {}", config.blk_file_path);

  let tasks: Vec<Box<dyn task_manager::Task>> = vec![
    Box::new(BlkFileImportTask::new()),
    Box::new(BlockHeightCalculationTask::new()),
    Box::new(AddressDeduplicationTask::new()),
    Box::new(BirConstructionTask::new()),
    Box::new(ClusteringTask::new()),
  ];
  let task_manager = task_manager::TaskManager::new(config, tasks);
  if let Err(error) = task_manager.run() {
    error!("{}", error);
    error!("{}", error.backtrace());
  } else {
    info!("Finished import.");
  }
}

fn configure_logger(matches: &clap::ArgMatches) {
  let log_level = if matches.is_present("debug") {
    LogLevelFilter::Debug
  } else {
    LogLevelFilter::Info
  };
  SimpleLogger::init(log_level, simplelog::Config::default()).unwrap();
}
