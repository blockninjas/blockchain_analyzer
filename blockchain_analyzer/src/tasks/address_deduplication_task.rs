use config::Config;
use db_persistence::repository::*;
use diesel::{self, prelude::*};
use {Index, Task};

pub struct AddressDeduplicationTask {}

impl AddressDeduplicationTask {
  pub fn new() -> AddressDeduplicationTask {
    AddressDeduplicationTask {}
  }
}

impl Task for AddressDeduplicationTask {
  fn run(&self, _config: &Config, db_connection: &PgConnection) {
    info!("Deduplicate addresses");
    db_connection
      .transaction::<(), diesel::result::Error, _>(|| {
        let address_deduplicator_state_repository =
          AddressDeduplicatorStateRepository::new(db_connection);

        let latest_deduplicated_output_address_id =
          match address_deduplicator_state_repository.latest() {
            Some(id) => id,
            None => 0,
          };

        let address_repository = AddressRepository::new(db_connection);
        address_repository
          .deduplicate_output_addresses(latest_deduplicated_output_address_id);

        let output_address_repository =
          OutputAddressRepository::new(db_connection);
        let max_output_address_id = output_address_repository.max_id().unwrap();

        address_deduplicator_state_repository.save(max_output_address_id);
        Ok(())
      })
      .unwrap();
  }

  fn get_indexes(&self) -> Vec<Index> {
    vec![Index {
      table: String::from("addresses"),
      column: String::from("base58check"),
      unique: true,
    }]
  }
}
