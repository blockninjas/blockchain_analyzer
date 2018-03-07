extern crate blk_file_reader;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
extern crate rayon;

pub mod repository;
pub mod schema;
pub mod domain;
mod blkfileimporter;

use rayon::prelude::*;
use blk_file_reader::list_blk_files;
use diesel::prelude::*;

use blkfileimporter::BlkFileImporter;

pub fn import_blk_files(path: &str, database_url: &str) -> std::io::Result<()> {
    let blk_files = list_blk_files(path)?;
    // TODO Make number of threads configurable.
    blk_files.par_iter()
        .map(|blk_file| import_blk_file(blk_file, database_url))
        .reduce_with(|r1, r2| {
            if r1.is_err() { r1 } else { r2 }
        })
        .unwrap_or(Ok(()))
}

fn establish_connection(database_url: &str) -> PgConnection {
    let connection = PgConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url));

    info!("Established database connection");

    connection
}

fn import_blk_file(blk_file_path: &str, database_url: &str) -> std::io::Result<()> {
    info!("Parse {}", blk_file_path);
    let db_connection = establish_connection(database_url);
    let _ = db_connection.transaction::<(), diesel::result::Error, _>(|| {
        let blk_file_importer = BlkFileImporter::new(&db_connection);
        blk_file_importer.import_blk_file(blk_file_path)
    }).unwrap();
    Ok(())
}
