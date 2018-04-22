//! # Build Script
//!
//! This build script ensures that `diesel` migrations are run before any tests
//! are invoked.

extern crate diesel;
extern crate diesel_migrations;
extern crate dotenv;

use diesel_migrations::run_pending_migrations;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

fn main() {
  println!("cargo:rerun-if-env-changed=TEST_DATABASE_URL");
  println!("cargo:rerun-if-changed=migrations/");
  if env::var("PROFILE") == Ok("debug".into()) {
    let _ = dotenv();
    if let Ok(database_url) = env::var("TEST_DATABASE_URL") {
      let connection = PgConnection::establish(&database_url)
        .expect("Could not connect to TEST_DATABASE_URL");
      run_pending_migrations(&connection).expect("Error running migrations");
    }
  }
}
