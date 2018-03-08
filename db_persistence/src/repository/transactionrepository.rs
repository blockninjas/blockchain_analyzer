use super::Repository;
use domain::Transaction;
use domain::NewTransaction;
use schema::transactions;
use diesel;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;

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
