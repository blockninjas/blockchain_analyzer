use diesel::{self, pg::PgConnection, RunQueryDsl};
use domain::{NewTransaction, Transaction};
use schema::transactions;
use std::result::Result;

pub fn save(
    db_connection: &PgConnection,
    new_transaction: &NewTransaction,
) -> Result<Transaction, diesel::result::Error> {
    diesel::insert_into(transactions::table)
        .values(new_transaction)
        .get_result(db_connection)
}
