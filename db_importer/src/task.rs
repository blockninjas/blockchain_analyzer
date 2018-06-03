use super::Index;
use config::Config;
use diesel::prelude::*;

/// A `Task` is a coherent set of actions to be executed during an import.
pub trait Task {
  /// Runs this `Task`.
  fn run(&self, config: &Config, db_connection: &PgConnection);

  /// Get the indexes that are under the responsibility of this `Task`.
  fn get_indexes(&self) -> Vec<Index>;
}
