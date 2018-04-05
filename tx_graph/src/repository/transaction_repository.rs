use std::io;
use domain::NewTransaction;
use domain::Transaction;

pub trait TransactionRepository {
  fn save(&self, new_transaction: &NewTransaction) -> io::Result<()>;

  fn read(&self, transaction_id: usize) -> io::Result<Transaction>;
}
