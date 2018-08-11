use diesel::{self, pg::PgConnection, RunQueryDsl};
use domain::{NewTransaction, Transaction};
use schema::transactions;
use std::result::Result;

pub struct TransactionRepository<'a> {
  connection: &'a PgConnection,
}

impl<'a> TransactionRepository<'a> {
  pub fn new(connection: &'a PgConnection) -> TransactionRepository<'a> {
    TransactionRepository { connection }
  }

  pub fn save(
    &self,
    new_transaction: &NewTransaction,
  ) -> Result<Transaction, diesel::result::Error> {
    diesel::insert_into(transactions::table)
      .values(new_transaction)
      .get_result(self.connection)
  }
}
