use super::{Index, Task};
use config::Config;
use db::BlkFile;
use diesel::{self, prelude::*};
use failure::Error;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use rayon::prelude::*;
use std::result::Result;

pub struct TaskManager {
    config: Config,
    tasks: Vec<Box<Task>>,
}

impl TaskManager {
    pub fn new(config: Config, tasks: Vec<Box<Task>>) -> TaskManager {
        TaskManager { config, tasks }
    }

    pub fn run(&self) -> Result<(), Error> {
        info!(
            "Run TaskManager using following config:\n{:#?}",
            self.config
        );

        let db_connection_manager = ConnectionManager::<PgConnection>::new(&self.config.db_url[..]);

        let db_connection_pool = Pool::builder()
            .max_size(self.config.max_db_connections)
            .build(db_connection_manager)?;

        {
            let db_connection = db_connection_pool.get()?;
            if is_initial_import(&db_connection)? {
                self.drop_all_indices(&db_connection)?;
            }
        }

        for task in self.tasks.iter() {
            task.run(&self.config, &db_connection_pool)?;
            // TODO Remove explicit dereferencing if deref coercion for `Box<Trait>`
            // is working (see rust-lang issue
            // https://github.com/rust-lang/rust/issues/22194).
            create_task_indexes(&**task, &db_connection_pool)?;
        }

        Ok(())
    }

    fn drop_all_indices(&self, db_connection: &PgConnection) -> Result<(), Error> {
        let indices = self
            .tasks
            .iter()
            .flat_map(|task| task.get_indexes().into_iter());

        for index in indices {
            let query = format!(
                "DROP INDEX IF EXISTS {}_{}_index;",
                index.table, index.column
            );
            info!("{}", query);
            diesel::sql_query(query).execute(db_connection)?;
        }

        Ok(())
    }
}

fn is_initial_import(db_connection: &PgConnection) -> Result<bool, Error> {
    Ok(BlkFile::count(db_connection)? == 0)
}

fn create_task_indexes(
    task: &Task,
    db_connection_pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<(), Error> {
    task.get_indexes().par_iter().for_each(|index| {
        // TODO Return error instead of panicking.
        let db_connection = db_connection_pool.get().unwrap();
        create_index(&index, &db_connection).unwrap();
    });
    Ok(())
}

fn create_index(index: &Index, db_connection: &PgConnection) -> Result<(), Error> {
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

    diesel::sql_query(query).execute(db_connection)?;

    Ok(())
}
