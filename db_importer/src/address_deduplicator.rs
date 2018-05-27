use db_persistence::repository::*;
use diesel::PgConnection;

pub fn deduplicate_output_addresses(db_connection: &PgConnection) {
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

  let output_address_repository = OutputAddressRepository::new(db_connection);
  let max_output_address_id = output_address_repository.max_id().unwrap();

  address_deduplicator_state_repository.save(max_output_address_id);
}

#[cfg(test)]
mod test {
  // TODO Add tests.
}
