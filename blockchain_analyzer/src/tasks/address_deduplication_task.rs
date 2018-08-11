use config::Config;
use db_persistence::repository::*;
use diesel::prelude::*;
use failure::Error;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::result::Result;
use task_manager::{Index, Task};

pub struct AddressDeduplicationTask {}

impl AddressDeduplicationTask {
    pub fn new() -> AddressDeduplicationTask {
        AddressDeduplicationTask {}
    }
}

impl Task for AddressDeduplicationTask {
    fn run(
        &self,
        _config: &Config,
        db_connection_pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<(), Error> {
        info!("Deduplicate addresses");

        let db_connection = db_connection_pool.get()?;

        db_connection
            .transaction(|| deduplicate_addresses(&db_connection))
            .unwrap();

        Ok(())
    }

    fn get_indexes(&self) -> Vec<Index> {
        vec![Index {
            table: String::from("addresses"),
            column: String::from("base58check"),
            unique: true,
        }]
    }
}

fn deduplicate_addresses(db_connection: &PgConnection) -> Result<(), Error> {
    let output_address_repository = OutputAddressRepository::new(&db_connection);
    if let Some(max_output_address_id) = output_address_repository.max_id()? {
        let address_deduplicator_state_repository =
            AddressDeduplicatorStateRepository::new(&db_connection);

        let latest_deduplicated_output_address_id =
            match address_deduplicator_state_repository.latest()? {
                Some(id) => id,
                None => 0,
            };

        let address_repository = AddressRepository::new(&db_connection);
        address_repository.deduplicate_output_addresses(latest_deduplicated_output_address_id)?;

        address_deduplicator_state_repository.save(max_output_address_id)?;
    }

    Ok(())
}
