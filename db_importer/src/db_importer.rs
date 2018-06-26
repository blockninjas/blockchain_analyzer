use super::{
  AddressDeduplicationTask, BirFileWriterTask, BlkFileImportTask,
  BlockHeightCalculationTask, ClusteringTask, Index, Task,
};
use config::Config;
use db_persistence::repository::*;
use diesel::{self, prelude::*};

pub struct DbImporter {
  config: Config,
  tasks: Vec<Box<Task>>,
}

impl DbImporter {
  pub fn new(config: Config) -> DbImporter {
    DbImporter {
      config,
      tasks: vec![
        Box::new(BlkFileImportTask::new()),
        Box::new(BlockHeightCalculationTask::new()),
        Box::new(AddressDeduplicationTask::new()),
        Box::new(BirFileWriterTask::new()),
        Box::new(ClusteringTask::new()),
      ],
    }
  }

  pub fn run(&self) {
    info!("Run DbImporter using following config:\n{:#?}", self.config);

    // TODO Return error instead of panicking.
    let db_connection = PgConnection::establish(&self.config.db_url).unwrap();

    if self.is_initial_import(&db_connection) {
      self.drop_all_indices(&db_connection);
    }

    for task in self.tasks.iter() {
      task.run(&self.config, &db_connection);
      // TODO Remove explicit dereferencing if deref coercion for `Box<Trait>`
      // is working (see rust-lang issue
      // https://github.com/rust-lang/rust/issues/22194).
      self.create_task_indexes(&**task, &db_connection);
    }
  }

  fn is_initial_import(&self, db_connection: &PgConnection) -> bool {
    let blk_file_repository = BlkFileRepository::new(&db_connection);
    blk_file_repository.count() == 0
  }

  fn drop_all_indices(&self, db_connection: &PgConnection) {
    self
      .tasks
      .iter()
      .flat_map(|task| task.get_indexes().into_iter())
      .for_each(|index| {
        let query = format!(
          "DROP INDEX IF EXISTS {}_{}_index;",
          index.table, index.column
        );
        info!("{}", query);
        diesel::sql_query(query).execute(db_connection).unwrap();
      });
  }

  fn create_task_indexes(&self, task: &Task, db_connection: &PgConnection) {
    for index in task.get_indexes() {
      self.create_index(&index, db_connection);
    }
  }

  fn create_index(&self, index: &Index, db_connection: &PgConnection) {
    let index_type = if index.unique {
      String::from("UNIQUE")
    } else {
      String::new()
    };

    let query = format!(
      "CREATE {index_type} INDEX IF NOT EXISTS {table}_{column}_index ON {table} ( {column} );",
      table = index.table,
      column = index.column,
      index_type = index_type,
    );

    info!("{}", query);

    diesel::sql_query(query).execute(db_connection).unwrap();
  }
}
