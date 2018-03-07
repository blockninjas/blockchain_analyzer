#[macro_use]
extern crate diesel;
extern crate blk_file_reader;

use diesel::prelude::*;

pub mod domain;
pub mod repository;
pub mod schema;

// TODO Move to meaningfully named sub-module.
pub fn establish_connection(database_url: &str) -> PgConnection {
    let connection = PgConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url));

    connection
}
