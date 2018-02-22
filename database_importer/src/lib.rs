#[macro_use]
extern crate diesel;
extern crate dotenv;

mod repository;
pub mod schema;
mod domain;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use repository::*;
use domain::*;


pub fn import_blk_files(_path: &str) -> usize {
    0
}

pub fn import_blk_file(_path: &str) -> usize {
    0
}
