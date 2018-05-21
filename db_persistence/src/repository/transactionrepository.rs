use super::Repository;
use diesel;
use diesel::RunQueryDsl;
use diesel::pg::PgConnection;
use domain::NewTransaction;
use domain::Transaction;
use schema::transactions;

pub struct TransactionRepository<'a> {
  connection: &'a PgConnection,
}

impl<'a> TransactionRepository<'a> {
  pub fn new(connection: &'a PgConnection) -> TransactionRepository<'a> {
    TransactionRepository { connection }
  }
}

impl<'a> Repository for TransactionRepository<'a> {
  type NewEntity = NewTransaction;
  type Entity = Transaction;

  fn save(&self, new_transaction: &NewTransaction) -> Transaction {
    diesel::insert_into(transactions::table)
      .values(new_transaction)
      .get_result(self.connection)
      .expect("Error saving new transaction")
  }
}
