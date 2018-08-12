use diesel::{self, dsl::max, pg::PgConnection, QueryDsl, RunQueryDsl};
use schema::output_addresses::dsl::*;
use std::result::Result;

#[derive(Queryable)]
pub struct OutputAddress {
    pub output_id: i64,
    pub hash: Vec<u8>,
    pub base58check: String,
}

impl OutputAddress {
    /// Returns the maximal output address id, or `None` if no address exists yet.
    pub fn max_id(db_connection: &PgConnection) -> Result<Option<i64>, diesel::result::Error> {
        // TODO Return error instead of panicking.
        output_addresses.select(max(output_id)).first(db_connection)
    }
}
