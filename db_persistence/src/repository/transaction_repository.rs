use diesel::{self, pg::PgConnection, RunQueryDsl};
use domain::{NewTransaction, Transaction};
use schema::transactions;

pub struct TransactionRepository<'a> {
  connection: &'a PgConnection,
}

impl<'a> TransactionRepository<'a> {
  pub fn new(connection: &'a PgConnection) -> TransactionRepository<'a> {
    TransactionRepository { connection }
  }

  pub fn save(&self, new_transaction: &NewTransaction) -> Transaction {
    diesel::insert_into(transactions::table)
      .values(new_transaction)
      .get_result(self.connection)
      .expect("Error saving new transaction")
  }
}
