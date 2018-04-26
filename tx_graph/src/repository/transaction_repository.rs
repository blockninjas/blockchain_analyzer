use std::io;
use domain::NewTransaction;
use domain::Transaction;

pub trait TransactionRepository {
  fn save(&self, new_transaction: &NewTransaction) -> io::Result<()>;
  fn read(&self, hash: &[u8; 32]) -> io::Result<Transaction>;
}
