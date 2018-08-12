use config::Config;
use db_persistence::{domain::*, schema};
use diesel::{self, prelude::*, sql_query};
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
            .transaction(|| deduplicate_addresses_and_save_state(&db_connection))
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

fn deduplicate_addresses_and_save_state(db_connection: &PgConnection) -> Result<(), Error> {
    if let Some(max_output_address_id) = OutputAddress::max_id(&db_connection)? {
        let latest_deduplicated_output_address_id = match read_latest_state(db_connection)? {
            Some(id) => id,
            None => 0,
        };

        deduplicate_addresses(db_connection, latest_deduplicated_output_address_id)?;

        save_state(db_connection, max_output_address_id)?;
    }

    Ok(())
}

fn deduplicate_addresses(
    db_connection: &PgConnection,
    latest_deduplicated_output_address_id: i64,
) -> Result<usize, diesel::result::Error> {
    let query = format!(
        r"
        insert into addresses (base58check)
          select base58check from output_addresses
            where output_addresses.output_id > {}
            group by base58check
          on conflict do nothing
      ",
        latest_deduplicated_output_address_id
    );

    // TODO Return error instead of panicking.
    sql_query(query).execute(db_connection)
}

/// Saves a new address deduplicator state with the given `output_address_id`.
fn save_state(
    db_connection: &PgConnection,
    new_output_address_id: i64,
) -> Result<usize, diesel::result::Error> {
    // TODO Return error instead of panicking.
    diesel::insert_into(schema::address_deduplicator_states::table)
        .values(
            schema::address_deduplicator_states::dsl::output_address_id.eq(new_output_address_id),
        )
        .execute(db_connection)
}

/// Returns the id of the latest deduplicated output address or `None` if none
/// have been deduplicated so far.
fn read_latest_state(db_connection: &PgConnection) -> Result<Option<i64>, diesel::result::Error> {
    // TODO Return error instead of panicking.
    schema::address_deduplicator_states::table
        .select(schema::address_deduplicator_states::dsl::output_address_id)
        .order(schema::address_deduplicator_states::dsl::id.desc())
        .first(db_connection)
        .optional()
}
