extern crate blk_file_reader;
extern crate db_persistence;
extern crate diesel;
#[macro_use]
extern crate log;
extern crate bincode;
extern crate bir;
extern crate bir_construction;
extern crate clustering;
extern crate config;
extern crate rayon;

mod address_deduplication_task;
mod bir_file_writer_task;
mod blk_file_import_task;
mod blk_file_importer;
mod block_height_calculation_task;
mod clustering_task;
mod db_importer;
mod index;
mod task;

use address_deduplication_task::AddressDeduplicationTask;
use bir_file_writer_task::BirFileWriterTask;
use blk_file_import_task::BlkFileImportTask;
use block_height_calculation_task::BlockHeightCalculationTask;
use clustering_task::ClusteringTask;
use index::Index;
use task::Task;

pub use db_importer::DbImporter;
