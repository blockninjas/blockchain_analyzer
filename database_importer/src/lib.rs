extern crate blk_file_reader;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate rayon;

pub mod repository;
pub mod schema;
pub mod domain;
mod blkfileimporter;

use dotenv::dotenv;
use std::env;
use rayon::prelude::*;
use blk_file_reader::list_blk_files;
use diesel::prelude::*;

use blkfileimporter::BlkFileImporter;

pub fn import_blk_files(path: &str) -> std::io::Result<()> {
    let blk_files = list_blk_files(path)?;

    info!("num threads: {}", rayon::current_num_threads());

    blk_files.par_iter()
        .map(import_blk_file)
        .reduce_with(|r1, r2| {
            if r1.is_err() { r1 } else { r2 }
        })
        .unwrap_or(Ok(()))
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    info!("Established database connection");

    connection
}

fn import_blk_file(blk_file_path: &String) -> std::io::Result<()> {
    info!("Parse {}", blk_file_path);
    let db_connection = establish_connection();
    let _ = db_connection.transaction::<(), diesel::result::Error, _>(|| {
        let blk_file_importer = BlkFileImporter::new(&db_connection);
        blk_file_importer.import_blk_file(blk_file_path)
    }).unwrap();
    Ok(())
}
