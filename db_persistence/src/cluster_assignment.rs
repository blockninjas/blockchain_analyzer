use diesel::{self, prelude::*};
use schema;
use std::result::Result;

#[derive(Queryable)]
pub struct ClusterAssignment {
    pub id: i64,
    pub cluster_representative: Option<i64>,
}

impl ClusterAssignment {
    pub fn load_in_range(
        db_connection: &PgConnection,
        address_id: i64,
        count: i64,
    ) -> Result<Vec<ClusterAssignment>, diesel::result::Error> {
        schema::addresses::dsl::addresses
            .select((
                schema::addresses::dsl::id,
                schema::addresses::dsl::cluster_representative,
            ))
            .filter(
                schema::addresses::dsl::id
                    .ge(address_id)
                    .and(schema::addresses::dsl::id.lt(address_id + count)),
            )
            .get_results(db_connection)
    }
}
