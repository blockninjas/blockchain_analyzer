#![allow(proc_macro_derive_resolution_fallback)]

extern crate bit_vec;
extern crate blk_file_reader;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate union_find;
#[macro_use]
extern crate diesel;
extern crate lru_cache;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate bincode;
extern crate failure;
extern crate rayon;
extern crate serde;

mod bir;
mod config;
mod db;
pub mod task_manager;
pub mod tasks;

pub use config::Config;
use db::schema;
