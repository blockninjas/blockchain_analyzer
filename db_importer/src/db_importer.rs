use super::{AddressDeduplicationTask, BlkFileImportTask,
            BlockHeightCalculator, ClusteringTask, Index, Task};
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
        // TODO Fix block height calculation query - currently the query does
        // not finish on `sample_blk_files/blk00000.dat` and seems to execute
        // endlessly.
        // Box::new(BlockHeightCalculator::new()),
        Box::new(AddressDeduplicationTask::new()),
        Box::new(ClusteringTask::new()),
      ],
    }
  }

  pub fn run(&self) {
    info!("run");

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
          "DROP INDEX IF EXISTS {}_{}_index",
          index.table, index.column
        );
        info!("{}", query);
        diesel::sql_query(query)
          .execute(db_connection)
          .unwrap();
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
      "CREATE {index_type} INDEX IF NOT EXISTS {table}_{column}_index ON {table} ( {column} )",
      table = index.table,
      column = index.column,
      index_type = index_type,
    );

    info!("{}", query);

    diesel::sql_query(query)
      .execute(db_connection)
      .unwrap();
  }
}

//fn get_all_index_columns() -> Vec<Index> {
//  vec![
//    Index {
//      table: String::from("blocks"),
//      column: String::from("hash"),
//      unique: false,
//    },
//    Index {
//      table: String::from("blocks"),
//      column: String::from("height"),
//      unique: false,
//    },
//    Index {
//      table: String::from("blocks"),
//      column: String::from("previous_block_hash"),
//      unique: false,
//    },
//    Index {
//      table: String::from("transactions"),
//      column: String::from("block_id"),
//      unique: false,
//    },
//    Index {
//      table: String::from("transactions"),
//      column: String::from("hash"),
//      unique: false,
//    },
//    Index {
//      table: String::from("inputs"),
//      column: String::from("transaction_id"),
//      unique: false,
//    },
//    Index {
//      table: String::from("inputs"),
//      column: String::from("previous_tx_hash"),
//      unique: false,
//    },
//    Index {
//      table: String::from("outputs"),
//      column: String::from("transaction_id"),
//      unique: false,
//    },
//    Index {
//      table: String::from("output_addresses"),
//      column: String::from("base58check"),
//      unique: false,
//    },
//    Index {
//      table: String::from("addresses"),
//      column: String::from("base58check"),
//      unique: false,
//    },
//  ]
//}