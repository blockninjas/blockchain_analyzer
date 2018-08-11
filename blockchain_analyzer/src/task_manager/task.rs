use super::Index;
use config::Config;
use diesel::prelude::*;
use failure::Error;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::result::Result;

/// A `Task` is a coherent set of actions to be executed during an import.
pub trait Task {
    /// Runs this `Task`.
    fn run(
        &self,
        config: &Config,
        db_connection_pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<(), Error>;

    /// Get the indexes that are under the responsibility of this `Task`.
    fn get_indexes(&self) -> Vec<Index>;
}
